# FS SIMULATOR

HOW:
1. Using BST to maintain the order of subdirectories
2. Keep track default directory
3. Each node has a list of subdirectories which are pairs, with the key being the subdirectory and the value being the BST of its child

RUN:

test1:
```
$ cargo run < input1.txt && cat output.txt

Command: dir
Directory of root:
No subdirectories
Command: mkdir   sub6
Command: mkdir   sub3
Command: mkdir   sub4
Command: dir
Directory of root:
sub3    sub4    sub6
Command: mkdir   sub4
Subdirectory already exists
Command: cd      sub3
Command: cd      sub3
Subdirectory does not exist
Command: mkdir   sub3
Command: mkdir   sub6
Command: mkdir   sub4
Command: dir
Directory of root\sub3:
sub3    sub4    sub6
Command: cd      sub6
Command: mkdir   sub666
Command: dir
Directory of root\sub3\sub6:
sub666
Command: up
Command: up
Command: dir
Directory of root:
sub3    sub4    sub6
Command: up
Cannot move up from root directory
```

test2:
```
$ cargo run < input2.txt && cat output.txt

Command: mkdir   sub3
Command: mkdir   sub4
Command: tree
Tree of root:
.
├── sub3
└── sub4
Command: mv      sub3    sub6
Command: cd      sub3
Subdirectory does not exist
Command: cd      sub6
Command: mkdir   sub601
Command: mv      sub601  ..\sub4
Command: up
Command: tree
Tree of root:
.
├── sub4
│   └── sub601
└── sub6
Command: cd      sub4
Command: cd      sub601
Command: mkdir   sub666
Command: up
Command: mv      sub601  sub602
Command: cd      sub602
Command: cd      sub666
Command: tree
Tree of root\sub4\sub602\sub666:
.
Command: up
Command: up
Command: mkdir   sub6
Command: up
Command: tree
Tree of root:
.
├── sub4
│   ├── sub6
│   └── sub602
│       └── sub666
└── sub6
Command: mv      sub6    sub4
Subdirectory already exists
```
