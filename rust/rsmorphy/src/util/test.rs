pub fn init_logger() {
    use std::sync::Once;

    static INIT: Once = Once::new();

    INIT.call_once(|| {
        env_logger::init();
    });
}
