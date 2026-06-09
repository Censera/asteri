; ModuleID = 'main'
source_filename = "main"

@.str = private unnamed_addr constant [14 x i8] c"Hello, world!\00", align 1
@fmt = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %printf = call i32 (ptr, ...) @printf(ptr @fmt, ptr @.str)
  ret i32 0
}
