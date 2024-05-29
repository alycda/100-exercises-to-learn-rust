pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    const my_mem:usize = size_of::<usize>();

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), my_mem * 3); // 24 - pointer: usize, length: usize capacity: usize
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Data layout" section of the Rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.
        assert_eq!(size_of::<Ticket>(), my_mem * 3 * 3); // 72 I don't know why "72" is intuitive here. I guess it's because the struct has 3 strings...
    }
}
