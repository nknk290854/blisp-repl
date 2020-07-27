use linked_list_allocator;
use linked_list_allocator::LockedHeap;

//#[global_allocator]
//static ALLOCATOR: LockedHeap = LockedHeap::empty();


pub fn init_heap(start: usize, end: usize) {
    let heap_start = start;
    let heap_end = end;
    let heap_size = heap_end - heap_start;
//    unsafe {
  //        ALLOCATOR.lock().init(heap_start, heap_size);
    //}
}