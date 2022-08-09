use std::cell::{RefCell, RefMut};
struct Health(i32);
struct Name(&'static str);

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}

struct World {
    // We'll use `entities_count` to assign each Entity a unique ID.
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl World {
    fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        self.component_vecs
            .iter_mut()
            .for_each(|component_vec| {
                component_vec.push_none();
            });

        self.entities_count += 1;
        entity_id
    }

    fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(c_v) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>() 
            {
                c_v.borrow_mut()[entity] = Some(component);
                return;
            }
        }

        // No matching component storage exists yet
        let mut new_cv: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);
        
        for _ in 0..self.entities_count {
            new_cv.push(None);
        }

        new_cv[entity] = Some(component);
        self.component_vecs.push(Box::new(RefCell::new(new_cv)));
    }

    fn borrow_component_vec_mut<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }

        None
    }
}

fn main() {
    let mut world = World::new();

    let icarus_entity = world.new_entity();
    world.add_component_to_entity(icarus_entity, Name("Icarus"));
    world.add_component_to_entity(icarus_entity, Health(-10));

    let prometheus_entity = world.new_entity();
    world.add_component_to_entity(icarus_entity, Name("Prometheus"));
    world.add_component_to_entity(icarus_entity, Health(100));

    let zeus_entity = world.new_entity();
    world.add_component_to_entity(zeus_entity, Name("Zeus"));

    let mut healths = world.borrow_component_vec_mut::<Health>().unwrap();
    let mut names = world.borrow_component_vec_mut::<Name>().unwrap();
    let zip = healths.iter_mut().zip(names.iter_mut());
    let iter = zip.filter_map(|(health, name)| Some((health.as_mut()?, name.as_mut()?)));

    for (health, name) in iter {
        if health.0 < 0 {
            println!("{} has perished", name.0)
        }

        if name.0 == "Perseus" && health.0 <= 0 {
            *health = Health(100);
        }
    }
}