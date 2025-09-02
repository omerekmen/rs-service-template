# rs-service-template — Clean Architecture + DDD Taslak Yapısı (Netleştirilmiş)

Bu doküman; Rust ile Clean Architecture ve Domain-Driven Design (DDD) prensiplerine uygun, katmanlı (Domain, Application, Infrastructure, Presentation) ve ortak bileşenler (Shared) içeren bir proje iskeletini tanımlar. Amaç; domain kurallarını merkezde tutan, test edilebilir, değişime dayanıklı ve teknolojilerle (HTTP, DB, Cache, MQ) gevşek bağlı bir mimari kurmaktır.

Karar özeti:
- HTTP Framework: Actix Web (performans ve olgun ekosistem tercih edildi)
- Yapı: Cargo workspace (çoklu crate) — tek crate opsiyonu kaldırıldı
- Veritabanı: Postgres + SQL Server desteği (sqlx ana aday; MSSQL için sqlx mssql veya tiberius fallback)
- MQ: RabbitMQ (lapin) öncelikli, opsiyonel Kafka
- Kimlik/Yetki: JWT, JWKS, OIDC/OAuth2, RBAC destekli şablon
- API versiyonlama: izin verilen/deprecated yönetimi + opsiyonel RBAC temelli versiyon erişimi

---

## Hedefler ve İlkeler

- Domain merkezli: İş kuralları (Entities, Value Objects, Policies) teknoloji yığınından bağımsız.
- Bağımlılık yönü tek: Presentation → Application → Domain; Infrastructure, Application/Domain portlarını uygular; kompozisyon kökte yapılır.
- Arayüzler üzerinden iletişim: Port/Adapter (Hexagonal) yaklaşımı.
- Test edilebilirlik: Domain saf, Application fakes ile; E2E testler container’larla.
- Evrilebilirlik: Yeni adapter (örn. farklı DB) eklemek tek noktadan yapılabilir.

---

## Katmanlar ve Sorumluluklar

- Domain (core):
  - Entities, Value Objects, Domain Services, Domain Events.
  - Domain hataları (DomainError) ve kuralları.
  - Teknoloji bağımsızdır; serde dışı bağımlılıklardan kaçınılır.

- Application (use cases):
  - Use-case orchestrasyonu (Command/Query, Service/Handler).
  - Portlar (Repository, Cache, EventBus, Clock, IdGenerator vb.).
  - DTO/Contract (isteğe bağlı) ve mapping.
  - Domain ve Shared’a bağımlıdır; Infrastructure’a bağımlı değildir.

- Infrastructure (adapters):
  - Port’ların somut implementasyonları: DB (sqlx/Postgres), Cache (Redis), MQ (RabbitMQ/lapin), Config, Telemetry.
  - Migrasyonlar ve dış sistem konfigürasyonları.
  - Application/Domain bağımsızdır; sadece portları uygular.

- Presentation (API):
  - HTTP katmanı (Actix Web — yüksek performans, olgun middleware ve extractor ekosistemi).
  - Route’lar, middleware’ler, error → HTTP mapping (Responder/ResponseError).
  - Kompozisyon kökü: somut adapter’ları bağlar, DI/constructor ile handler’lara verir.

- Shared (ortak):
  - Ortak tipler: AppError, Result alias, base DTO’lar, Pagination, Id/Clock abstraction, Tracing helpers.
  - Katmanlar arası paylaşılabilir küçük utilities.

---

## Yapı Yaklaşımı: Cargo workspace ile çoklu crate (Seçildi)

Önerilen crates:
```
crates/
  shared/           # lib
  domain/           # lib (depends on: shared)
  application/      # lib (depends on: domain, shared)
  infrastructure/   # lib (depends on: application, domain, shared)
  presentation/ # bin/lib (depends on: application, shared, infrastructure)
```
Kök `Cargo.toml` (workspace) ile bağımlılık ve feature’lar yönetilir.

Artılar:
- Katmanlar arası sınırlar derleyici ile korunur.
- Yeniden kullanılabilirlik ve bağımsız versiyonlama kolaylaşır.
- CI cache verimliliği yüksek, test ve derleme daha kontrollü.

Eksiler:
- Başlangıç karmaşıklığı biraz daha yüksek.
- Birden çok `Cargo.toml` ve publish/build senaryoları.

Ne zaman?
- Orta-büyük servisler, uzun ömürlü kurumsal kod tabanları.


Bağımlılık grafiği (ok yönü: "bağımlıdır"):
```
domain   <- application <- presentation
   ^             ^              |
   |             |              v
   +--------- infrastructure ---+
```
- Application, Infrastructure’a referans vermez.
- Infrastructure; Application/Domain’daki portları uygular.
- presentation kompozisyon köküdür, somut adapter’ları bağlar.

---

## Port/Adapter (Hexagonal) Tasarımı

- Ports (Application):
  - RepositoryPort<T>, UnitOfWork (opsiyonel), CachePort, EventBusPort, Clock, IdGenerator, SecretsProvider.
- Adapters (Infrastructure):
  - SqlxPostgresRepository, SqlxMssqlRepository, RedisCache, RabbitMqEventBus (lapin), SystemClock, UlidGenerator, EnvSecretsProvider.
- Composition (Presentation):
  - Actix Web bootstrap: pools, clients, adapters oluşturulur ve handler’lara enjekte edilir.

---

## Hata Yönetimi ve Sonuç Tipleri

- Shared::error:
  - `AppError` (thiserror) — katmanlar arası standartlaştırılmış hata.
  - `type AppResult<T> = Result<T, AppError>`.
- Domain özel hataları `DomainError` ile ayrılabilir ve `From<DomainError> for AppError` uygulanır.
- Presentation: `impl IntoResponse for AppError` ile HTTP status/JSON body mapping.

---

## DTO’lar, Request/Response ve Mapping

- Application layer’da Command/Query DTO’ları (serde) ve domain mapping (From/TryFrom) önerilir.
- Presentation sadece HTTP/transport kaygıları (headers, query, path) ile ilgilenir, Application DTO’larına map eder.
- Dış dünya ile paylaşılan contract’lar breaking-change kuralları ile versiyonlanabilir (örn. `/v1`).

---

## Teknoloji Seçimleri ve Notlar

- HTTP Framework: Actix Web (yüksek performans, olgun middleware/extractor ekosistemi).
- DB Erişimi: sqlx (Postgres + MSSQL) önerilir.
  - Artılar: async-native, compile-time query check (offline mode), geniş kullanım, ORM kilitlemesi yok.
  - MSSQL: `sqlx` mssql backend kullanılabilir; belirli özelliklerde kısıt olursa `tiberius` fallback’i önerilir.
  - Alternatifler: Diesel (Postgres/MySQL/SQLite — MSSQL yok), SeaORM (yüksek seviye ORM, birçok backend; ağır sorgularda ham SQL’e dönülebilir), SurrealDB (spesifik ihtiyaç).
  - Öneri: Hibrit yaklaşım — CRUD benzeri akışlarda query builder/ORM; ağır rapor/join’lerde el yazımı SQL (sqlx). Repository portları bu esnekliği saklar.
- Cache: Redis (`redis`/`deadpool-redis` veya `fred`).
- MQ/Eventing: RabbitMQ (lapin) veya Kafka (rdkafka) — use-case’e göre; transactional outbox deseni önerilir.
- Config: `config` (+ serde) veya `figment`.
- Observability: `tracing`, `tracing-subscriber`, opsiyonel `opentelemetry` + OTLP.
- Runtime: `tokio`.
- Kimlik & Zaman: ULID/UUID (`ulid`, `uuid`), `time` veya `chrono` (uyumluluk ihtiyacına göre).

Performans Notu (Actix vs Axum):
- Sentetik benchmark’larda Actix çoğu zaman bir miktar öndedir; Axum ise ergonomi ve tower ekosistemiyle üretkenliği artırır.
- Her iki framework de prod için fazlasıyla yeterli performansa sahiptir; tercih ekosistem ve ekip alışkanlığına göre yapılmalıdır. Bu projede Actix standart kabul edilmiştir.

---

## Örnek Modülleşme (Workspace önerisi)

```
crates/
    shared/
        src/
            error.rs        # AppError, AppResult
            types.rs        # Pagination, Id types, common newtypes
            tracing.rs      # tracing setup helpers
            lib.rs

    domain/
        src/
            model/          # Entities, VOs
            service/        # Domain services
            rules/          # Policies, invariants
            error.rs        # DomainError
            lib.rs

    application/
        src/
            ports/
                repository.rs # Traits
                cache.rs
                event_bus.rs
                clock.rs
                id_gen.rs
            usecases/
                user/
                    create_user.rs
                    get_user.rs
            dto/
                user.rs       # Command/Query DTOs
            error.rs        # Application-level errors (optional)
            lib.rs

    infrastructure/
        src/
            db/
                mod.rs
                pg.rs         # sqlx Postgres repo impls
                mssql.rs
                migrate.rs
            cache/
                redis.rs
            mq/
                rabbitmq.rs
            config/
                settings.rs
            telemetry/
                tracing.rs
            lib.rs
        migrations/       # sqlx migrate files

    presentation/
        src/
            routes/
                users.rs
            middleware/
                auth.rs
            error_mapper.rs   # AppError -> ResponseError/Responder mapping
            bootstrap.rs        # Build adapters, wire dependencies
            main.rs             # Actix Web server start

---

## Kimlik Doğrulama, Yetkilendirme (Auth/RBAC)

- Tokenlar: JWT (access/refresh), JTI ile tekillik; `aud`, `iss`, `exp`, `nbf`, `sub`, `scope`/`roles` claim’leri.
- Doğrulama: JWKS üzerinden imza doğrulama (OIDC discovery destekli). Anahtar rotasyonu ve cache.
- Flow’lar: OAuth2/OIDC (Auth Code + PKCE öncelikli). Client Credentials servisler arası çağrılar için.
- RBAC: Role ve permission tabanlı; gerekirse ABAC (attribute-based) genişletmesi. Route-guard middleware (Actix extractor) ile enforcement.
- Multi-tenant: `tenant_id` claim’i ile izolasyon; repo ve cache katmanında tenant-aware erişim.
- Service-to-service: mTLS opsiyonel; kısmen SPIFFE/SPIRE entegrasyonu düşünülebilir.
- Güvenlik sertleri: Rate limit, brute-force koruması (Redis-based), ip allow/deny list, CORS politikaları.

Shared/crates önerileri:
- `shared::auth`: token doğrulama, jwks client, claim tipleri.
- `presentation::middleware::auth`: extractor + guard.

---

## API Versiyonlama ve Deprecation Politikası

- Versiyonlama: Yol temelli `/api/v1/...` (açık ve yönetilebilir). İsteğe bağlı olarak `Accept: application/vnd.<svc>.v2+json` header desteği eklenebilir.
- Deprecation: Yanıt header’ları ile (`Deprecation`, `Sunset`, `Link` ile dokümana yönlendirme), log/metric ile kullanım izleme.
- EOL süreci: v1 → deprecated duyurusu, belirlenen tarihte devre dışı; kritik akışlar için feature flag ile kademeli kapatma.
- RBAC-temelli versiyon erişimi: Belirli rollerin yeni versiyonu kullanmasına izin, diğerlerine v1 fallback (gateway/presentation katmanında policy ile).

Sözleşme yönetimi:
- Backward compatible değişiklik kuralları; breaking değişiklikte yeni major versiyon.
- Error şeması standart: `{ code, message, details?, trace_id }`.

---

## Çift Veritabanı Stratejisi (Postgres + SQL Server)

Hedef: Aynı domain/application portlarını koruyarak iki farklı RDBMS’i desteklemek.

- Özellik bayrakları (features): `db-pg`, `db-mssql`.
- Repository implementasyonları:
  - `infrastructure::db::pg::*` (sqlx Postgres)
  - `infrastructure::db::mssql::*` (sqlx mssql; ihtiyaç halinde `tiberius` fallback)
- Bağlantı havuzu: `sqlx::Pool<Postgres>` ve `sqlx::Pool<Mssql>` tip ayrışması; trait obje veya generic ile soyutlanır.
- Migration:
  - Ayrı klasörler: `migrations/pg` ve `migrations/mssql` — şematik farklar ayrı tutulur.
  - Versiyon eşliği: Her iki DB için semantik olarak eş değer migration’lar.
- Unit of Work (isteğe bağlı): Transaction boundary’i application seviyesinden kontrol etmek için trait tabanlı UoW; sqlx transaction wrapper.
- ORM/Query stratejisi:
  - CRUD ve basit sorgular: sqlx query builder / derive ile okunabilirlik.
  - Ağır join/rapor: el yazımı SQL (sqlx) — DB’ye özgü optimizasyon.
- Compile-time query check: sqlx offline mode (CI’de `SQLX_OFFLINE=true`) ile.

Karar rehberi:
- MSSQL tarafında sqlx’ın desteklemediği bir özellik ile karşılaşılırsa tiberius ile dar adapter yazılır ve port korunur.

---

## Event-Driven Mimarî (MQ)

- Broker: RabbitMQ (lapin). Alternatif: Kafka (yüksek hacim/partition ihtiyacı). 
- Pattern’ler:
  - Outbox: DB transaction ile aynı anda outbox tablosuna yaz; ayrı worker publish eder (en az bir kez teslimat, tutarlılık).
  - Idempotent consumer: `message_id`/`dedup_key` ile tüketici tarafında tekrarları yutma.
  - DLQ (Dead Letter Queue): Hatalı mesajlar için; retry policy, backoff.
  - SAGA/Process Manager: Çok adımlı süreçler için kompanzasyon.
- Mesaj şemaları: `event_name`, `version`, `occurred_at`, `payload`, `trace_id`.
- Telemetry: Mesaj publish/consume olaylarında tracing span’ları.

Crate yerleşimi:
- `application::ports::event_bus` — publish/subscribe arayüzleri.
- `infrastructure::mq::rabbitmq` — lapin tabanlı adapter.
- `infrastructure::outbox` — outbox tablo ve worker iskeleti.
```

Feature bayraklarıyla opsiyonellik:
- `infrastructure` crate: `db-postgres`, `cache-redis`, `mq-rabbitmq` gibi features.
- `presentation`: `otel`/`metrics` gibi opsiyonel telemetry features.

---

## Test Stratejisi

- Domain: saf unit test — dış bağımlılık yok.
- Application: port’lar için in-memory fake’lerle unit/integration.
- Infrastructure: gerçek adapter’lara karşı integration (sqlx migration + ephemeral DB).
- E2E: Axum server’ı testcontainers ile (Postgres, Redis, RabbitMQ) ayağa kaldırıp HTTP üzerinden senaryolar.

Önerilen crates:
- `testcontainers` (postgres, redis, rabbitmq modülleri),
- `serde_json`, `reqwest` (E2E), `insta` (snapshot/golden), `proptest` (property test) — isteğe bağlı.

---

## Versiyonlama, Hata Kodları ve Sözleşmeler

- API versiyonlama: `/api/v1/...`; breaking değişiklikte `/v2`.
- Error contract: `code`, `message`, `details`, `trace_id` alanlarıyla standardize.
- Pagination: `page`, `page_size`, `total`, `items` şeması.

---

## CI/CD Notları (Bu repo için hazır workflow’larla uyumlu)

- Lint/Format: `cargo fmt`, `clippy` (warnings as errors).
- Test: unit + integration; E2E opsiyonel.
- Security: `cargo-audit`, `cargo-deny` (lisans).
- Docker: `presentation` için çok aşamalı build + minimal runtime image.


## Sonraki Adımlar (Onay sonrası)

1. Workspace iskeletini oluşturma (crates klasörleri, Cargo.toml’lar, feature’lar).
2. Shared/AppError ve temel tipler.
3. Domain örnek entity/VO + temel kurallar.
4. Application port’ları ve 1-2 örnek use-case (Create/Get). 
5. Infrastructure: sqlx Postgres repo skeleton + migrations.
6. Presentation (Axum): health, metrics, örnek `POST /users`, `GET /users/{id}`.
7. E2E test için testcontainers altyapısı.

Onayınızla birlikte şablonu otomatik olarak oluşturup çalışır hale getirebilirim. 
