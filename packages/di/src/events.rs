use di_codegen::interface;

#[interface]
#[blackbox_di(local)]
pub trait OnModuleInit {
    async fn on_module_init(&self);
}

#[interface]
#[blackbox_di(local)]
pub trait OnModuleDestroy {
    async fn on_module_destroy(&self);
}
