use edgetpu::dep::uninstall::run_uninstall;
use edgetpu::dep::util::init_logger;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    run_uninstall()
}
