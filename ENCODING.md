# RKD Delta Encoding

RKD is a general delta encoding algorithm.

## Overview

The simlified version of how RKD delta encoding works:

1. Precompute rolling hashes of source and target file.
2. Iterate over source and target to specify source matches in target.
3. Encode matches and changes in delta file.

## Delta Instructions

- `ADD`: This instruction has two arguments, a size and a sequence of bytes.
- `COPY`: This instruction has two arguments, a size and a offset.

The delta is encoded using just `ADD` and `COPY`, `ADD` to specify a sequence of bytes to append, and `COPY` to specify a range of bytes in the source file to append. The operations are stored in the order that they should be applied to build the target so no target-related location information is required to apply the operations.

## Delta File Organization

The RKD delta file starts with a Header section which is followed by the Operation section. Below is the organization of the delta file where indented items refine the the ones immediately above them.

```txt
Header
    RKD delta file indicator       - 3 bytes
    RKD delta file version         - 2 bytes
    Decompressed target file size  - 4 bytes
Operations
    Operation1
    Operation2
    ...
```

Below is the organization of the operations where indented items refine the the ones immediately above them.

```txt
ADD
    Operation indicator            - 1 byte
    Sequence of bytes to add       - array of bytes
COPY
    Operation indicator            - 1 byte
    Offset to start the copy       - 4 bytes
    Number of bytes to copy        - 4 bytes
```

## Header Section

- RKD delta file indicator is `0x72 0x6B 0x64`.
- RKD delta file version has two bytes. The first byte indicates the major version and the second byte indicates the minor version. Backwards compatible changes are keep the same major version while incrementing the minor version.
- Decompressed target file size is the file size that the target will be after decompression.

## Operations section

The operations are ordered in the same order that they should be applied to generate the target file.

## Add Operation Section

- Operation indicator is zero to indicate the `ADD` operation.
- Sequence of bytes to add are the array of bytes to append to the target file.

## Copy Operation Section

- Operation indicator is one to indicate the `COPY` operation.
- Offset to start the copy is an unsigned 32 bit integer that indicates the source file offset to start the copy. The integer is saved as a four byte array in big-endian order.
- Number of bytes to copy is an unsigned 32 bit integer that indicates the number of bytes that should be copied, starting from the offset in the source file. The integer is saved as a four byte array in big-endian order.
