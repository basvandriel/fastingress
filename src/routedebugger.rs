use crate::{logger::Logger, route_entry::RouteEntry};

pub struct RouteDebugger {
    logger: Logger,
}

impl RouteDebugger {
    pub fn new(logger: Logger) -> RouteDebugger {
        RouteDebugger { logger }
    }

    pub fn debug(&self, routes: &[RouteEntry]) {
        if routes.is_empty() {
            return;
        }

        self.logger.info("Available routes:");
        println!();
        println!(
            "{0: <35} | {1: <15} | {2: <10} | {3: <20} | {4: <10}",
            "ingress_name", "host", "route", "service", "port"
        );

        for entry in routes.iter().clone() {
            println!(
                "{0: <35} | {1: <15} | {2: <10} | {3: <20} | {4: <10}",
                entry.ingress_name, entry.host, entry.route, entry.service, entry.port
            );
        }
        println!();
    }
}
