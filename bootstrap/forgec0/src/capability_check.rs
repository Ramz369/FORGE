/// Capability checking for the type system

use crate::ast::{Effect, Capability};

/// Check if one capability is subsumed by another
pub fn capability_subsumes(required: &Capability, provided: &Capability) -> bool {
    // Check effects using the lattice
    let required_effect = effect_join(&required.effects);
    let provided_effect = effect_join(&provided.effects);
    
    // Provided must be at least as permissive as required
    // In the lattice: Pure < Alloc < Io < Net
    // So Net can satisfy Io requirements
    if required_effect > provided_effect {
        return false;
    }
    
    // Check resource budgets
    check_resource_budgets(&required.budgets, &provided.budgets)
}

/// Join multiple effects into their least upper bound
pub fn effect_join(effects: &[Effect]) -> Effect {
    effects.iter()
        .fold(Effect::Pure, |acc, e| acc.join(e))
}

/// Check if provided budgets satisfy required budgets
fn check_resource_budgets(
    required: &crate::ast::ResourceBudget,
    provided: &crate::ast::ResourceBudget,
) -> bool {
    // Check token budget
    if let Some(req_tokens) = required.tokens {
        match provided.tokens {
            Some(prov_tokens) if prov_tokens >= req_tokens => {},
            _ => return false,
        }
    }
    
    // Check latency budget
    if let Some(req_latency) = required.latency_ms {
        match provided.latency_ms {
            Some(prov_latency) if prov_latency <= req_latency => {},
            _ => return false,
        }
    }
    
    // Check energy budget
    if let Some(req_energy) = required.energy_mj {
        match provided.energy_mj {
            Some(prov_energy) if prov_energy <= req_energy => {},
            _ => return false,
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ResourceBudget;
    
    #[test]
    fn test_effect_join_multiple() {
        let effects = vec![Effect::Pure, Effect::Alloc, Effect::Io];
        assert_eq!(effect_join(&effects), Effect::Io);
    }
    
    #[test]
    fn test_capability_subsumes_effects() {
        let required = Capability {
            effects: vec![Effect::Io],
            budgets: ResourceBudget { tokens: None, latency_ms: None, energy_mj: None },
        };
        
        let provided = Capability {
            effects: vec![Effect::Net],
            budgets: ResourceBudget { tokens: None, latency_ms: None, energy_mj: None },
        };
        
        // Net subsumes Io
        assert!(capability_subsumes(&required, &provided));
        
        // But Io doesn't subsume Net
        assert!(!capability_subsumes(&provided, &required));
    }
    
    #[test]
    fn test_resource_budget_checking() {
        let required = Capability {
            effects: vec![Effect::Pure],
            budgets: ResourceBudget { 
                tokens: Some(100),
                latency_ms: Some(50),
                energy_mj: None,
            },
        };
        
        let provided = Capability {
            effects: vec![Effect::Pure],
            budgets: ResourceBudget { 
                tokens: Some(200),  // More tokens OK
                latency_ms: Some(30), // Less latency OK
                energy_mj: Some(10),  // Extra constraint OK
            },
        };
        
        assert!(capability_subsumes(&required, &provided));
    }
}