# BlackBox DI

Dependency injection for Rust


## Install

Run the following Cargo command in your project directory:

```
cargo add blackbox_di
```

Or add the following line to your Cargo.toml:

```toml
blackbox_di = "0.1"
```

## How to use

### Provider creation

Annotate interface with `#[interface]`:

```rust
#[interface]
trait IService {
  fn call(&self);
}
```

Annotate service structure with `#[injectable]`:

```rust
#[injectable]
struct Service {}
```

Annotate impl block with `#[implements]`:

```rust
#[implements]
impl IService for Service {
  fn call() {
    println!("Service calling");
  }
}
```

### Module creation

Annotate module structure with `#[module]` and specify `Service` structure as a `provider`:

```rust
#[module]
struct RootModule {
  #[provider]
  service: Service
}
```

### Container creation

```rust
#[launch]
async fn launch() {
    let app = build::<RootModule>(BuildParams::default()).await;

    let service = app
      .get_by_token::<Service>(&get_token::<Service>)
      .unwrap();

    // short (equivalent to above)
    let service = app.get::<Service>().unwrap();
    service.call();
}
```

## Inject references

### Injecting by Type

Specify `#[inject]` for injectable dependencies:

```rust
#[injectable]
struct Repo {}

#[injectable]
struct Service {
  #[inject]
  repo: Ref<Repo>
}
```

Don't forget to specify the `Repo` in the `RootModule` module:

```rust
#[module]
struct RootModule {
  #[provider]
  repo: Repo

  #[provider]
  service: Service
}
```

### Injecting by Token

You can specify a token instead of a type:

```rust
#[injectable]
struct Repo {}

#[injectable]
struct Service {
  #[inject("REPO_TOKEN")]
  repo: Ref<Repo>
}
```
And then: 

```rust
#[module]
struct RootModule {
  #[provider("REPO_TOKEN")]
  repo: Repo

  #[provider]
  service: Service
}
```

Or use a constant as a token: 

```rust
const REPO_TOKEN: &str = "REPO_TOKEN";

#[injectable]
struct Repo {}

#[injectable]
struct Service {
  #[inject(REPO_TOKEN)]
  repo: Ref<Repo>
}

#[module]
struct RootModule {
  #[provider(REPO_TOKEN)]
  repo: Repo

  #[provider]
  service: Service
}
```

### Using interfaces 

You also can use interfaces for injectable dependencies:

```rust
const REPO_TOKEN: &str = "REPO_TOKEN";

#[interface]
trait IRepo {}

#[injectable]
struct Repo {}

#[implements]
impl IRepo for Repo {}

#[injectable]
struct Service {
  #[inject(REPO_TOKEN)]
  repo: Ref<dyn IRepo>
}

#[module]
struct RootModule {
  #[provider(REPO_TOKEN)]
  repo: Repo

  #[provider]
  service: Service
}
```

Or just use an existing implementation of the interface:

```rust
#[interface]
trait IRepo {}

#[injectable]
struct Repo {}

#[implements]
impl IRepo for Repo {}

#[injectable]
struct Service {
  #[inject(use Repo)]
  repo: Ref<dyn IRepo>
}

#[module]
struct RootModule {
  #[provider]
  repo: Repo

  #[provider]
  service: Service
}
```

## Factory 

If a service has non-injection dependencies:

```rust
#[injectable]
struct Service {
  #[inject]
  repo: Ref<Repo>

  greeting: String
}
```

You should specify a factory function:

```rust
#[implements]
impl Service {
  #[factory]
  fn new(repo: Ref<Repo>) -> Service {
    Service {
      repo, 
      greeting: String::from("Hello")
    } 
  }
}
```

Or for interfaces: 

```rust
#[injectable]
struct Service {
  #[inject(use Repo)]
  repo: Ref<dyn IRepo>

  greeting: String
}

#[implements]
impl Service {
  #[factory]
  fn new(repo: Ref<dyn IRepo>) -> Service {
    Service {
      repo, 
      greeting: String::from("Hello")
    } 
  }
}
```

Injectable services with `non-injectable` dependencies must have the `factory` functions.

To have mutable non-injectable deps, you need specify these dependencies with `RefMut<...>`:

```rust
#[injectable]
struct Service {
  #[inject(use Repo)]
  repo: Ref<dyn IRepo>

  greeting: RefMut<String>
}

#[implements]
impl Service {
  #[factory]
  fn new(repo: Ref<dyn Repo>) -> Service {
    Service {
      repo, 
      greeting: RefMut::new(String::from("Hello"))
    } 
  }

  fn set_greeting(&self, msg: String) {
    *self.greeting.as_mut() = msg;
  }

  fn print_greeting(&self) {
    println!("{}", self.greeting.as_ref());
  }
}
```

## Modules

You can specify multiple modules and import them:

```rust
#[module]
struct UserModule {
  #[provider]
  user_service: UserService,
}

#[module]
struct RootModule {
  #[import]
  user_module: UserModule
}
```

To use providers from imported modules you should specify these providers as `exported`:

```rust
#[module]
struct UserModule {
  #[provider]
  #[export]
  user_service: UserService,
}
```

Also, you can specify your modules as `global` then you don't have to import their directly. Just specify their only in the `root` module:

```rust
#[module]
#[global]
struct UserModule {
  #[provider]
  #[export]
  user_service: UserService,
}

#[module]
struct AccountModule {
  #[provider]
  #[export]
  account_service: AccountService,
}


#[module]
struct RootModule {
  #[import]
  user_module: UserModule

  #[import]
  account_module: AccountModule
}
```

## Dependency cycle

To resolve dependency cycle use `Lazy` when module importing:

```rust
#[module]
struct UserModule {
  #[import]
  account_module: Lazy<AccountService>,

  #[provider]
  #[export]
  user_service: UserService,
}

#[module]
struct AccountModule {
  #[import]
  user_module: Lazy<UserModule>,
  
  #[provider]
  #[export]
  account_service: AccountService,
}


#[module]
struct RootModule {
  #[import]
  user_module: UserModule

  #[import]
  account_module: AccountModule
}
```

## Lifecycle events

When the container is fully initialized, the system triggers events `on_module_init`:

```rust 
#[implements]
impl OnModuleInit for Service {
  async fn on_module_init(&self) {
    ...
  }
}
```

and `on_module_destroy`:

```rust 
#[implements]
impl OnModuleDestroy for Service {
  async fn on_module_destroy(&self) {
    ...
  }
}
```

## Logger

The logger is used to display information about app build. To use custom logger implement `ILogger` trait:

```rust 
pub trait ILogger {
    fn log<'a>(&self, level: LogLevel, msg: &'a str);
    fn log_with_ctx<'a>(&self, level: LogLevel, msg: &'a str, ctx: &'a str);
    fn set_context<'a>(&self, ctx: &'a str);
    fn get_context(&self) -> String;
}
```
And then change build params:

```rust

let app = build::<RootModule>(
  BuildParams::default().buffer_logs()
).await;

let custom_logger = app.get::<CustomLogger>().unwrap();

app.use_logger(custom_logger.cast::<dyn ILogger>().unwrap());
```

## License
BlackBox DI is licensed under:

- MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)