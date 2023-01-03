use std::collections::HashMap;

use casbin::{Enforcer, RbacApi};

pub struct CasbinService {
    pub enforcer: Enforcer,
}

impl CasbinService {
    pub fn get_permissions_for_role(&self, role: &str) -> HashMap<String, Vec<String>> {
        let permissions = self.enforcer.get_permissions_for_user(role, None);
        let mut result: HashMap<String, Vec<String>> = HashMap::new();
        for permission in permissions {
            let k = permission.get(2);
            let v = permission.get(1);
            if k.is_none() || v.is_none() {
                continue;
            }
            let key = k.expect("casbin error").clone();
            let value = result.entry(key).or_default();
            value.push(v.expect("casbin error").clone())
        }

        result
    }
}
