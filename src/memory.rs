use crate::math_core::Machine;

pub fn map_segment(machine: &mut Machine, size: u32) -> u32 {
    let new_segment = vec![0; size as usize];

    if let Some(id) = machine.free_ids.pop() {
        machine.segments[id as usize] = Some(new_segment);
        id
    } else {
        machine.segments.push(Some(new_segment));
        (machine.segments.len() - 1) as u32
    }
}

pub fn unmap_segment(machine: &mut Machine, id: u32) {
    if id == 0 {
        panic!("cannot unmap segment 0");
    }

    let index = id as usize;

    if index >= machine.segments.len() || machine.segments[index].is_none() {
        panic!("cannot unmap invalid segment");
    }

    machine.segments[index] = None;
    machine.free_ids.push(id);
}

pub fn segmented_load(machine: &Machine, segment_id: u32, offset: u32) -> u32 {
    machine.segments[segment_id as usize]
        .as_ref()
        .unwrap()[offset as usize]
}

pub fn segmented_store(machine: &mut Machine, segment_id: u32, offset: u32, value: u32) {
    machine.segments[segment_id as usize]
        .as_mut()
        .unwrap()[offset as usize] = value;
}