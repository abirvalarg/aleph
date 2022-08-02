#include "type.h"

#pragma GCC diagnostic ignored "-Wpointer-to-int-cast"
#pragma GCC diagnostic ignored "-Wint-to-pointer-cast"

struct Header
{
    u32 len     : 31;
    bool used   : 1;
};

_Static_assert(sizeof(struct Header) == 4, "");

void compact();

extern struct Header _STACK, _HEAP_END;

static struct Header *first_free = 0;

void heap_init()
{
    u32 len = ((u32)&_HEAP_END - (u32)&_STACK) / 4 - 2;
    _STACK.used = false;
    _STACK.len = len;

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Warray-bounds"
    struct Header *tail = &_HEAP_END - 1;
    tail->len = 0;
#pragma GCC diagnostic pop

    first_free = &_STACK;
}

void *alloc(u32 size, u32 align, bool try_compact)
{
    if(align < 4)
        align = 4;
    if(size % 4 || !size)
        size += 4 - size % 4;

    // find 1st free block
    struct Header *head = first_free;
    while(head->used && head->len)
        head += head->len + 1;

    struct Header *next;

    const u32 align_mask = align - 1;
    u32 *aligned;

    while(1)
    {
        // if reached end of heap...
        if(head->len == 0)
        {
            if(try_compact)
            {
                compact();
                return alloc(size, align, false);
            }
            else
                return 0;
        }
    
        // next block
       next = head + head->len + 1;

        // aligned address for data
        aligned = (u32*)((u32)head & ~align_mask);
        if(aligned <= (u32*)head)
            aligned += align / 4;

        // block has enough space to hold the aligned data
        if(aligned + size / 4 < (u32*)next)
            break;
        
        // looking for the next free block
        head = next;
        while(head->used && head->len)
            head += head->len + 1;
    }

    // unused space because of alignment
    u32 pad_space = ((u32)aligned - (u32)head) / 4 - 1;
    if(pad_space > 1)
    {
        head->len = pad_space - 1;
        head = (struct Header*)aligned - 1;
    }

    // header of the free segment block data and next block
    struct Header *mid = next;
    head->used = true;
    if(head->len > size / 4)
    {
        head->len = size / 4;

        mid = (struct Header*)aligned + size / 4;
        mid->len = ((u32)next - (u32)mid) / 4 - 1;
        mid->used = false;
    }

    if(head == first_free)
        first_free = mid;

    return aligned;
}

void dealloc(u32 *ptr)
{
    struct Header *head = (struct Header*)ptr - 1;
    head->used = false;

    struct Header *next = head + head->len + 1;
    if(!next->used && next->len)
        head->len += next->len + 1;
}

void compact()
{
    struct Header *head = &_STACK;
    while(head->len)
    {
        struct Header *next = head + head->len + 1;
        if(!head->used && !next->used && next->len)
            head->len += next->len + 1;
        else
        {
            head += head->len + 1;
            while(head->used)
                head += head->len + 1;
        }
    }
    first_free = &_STACK;
}
