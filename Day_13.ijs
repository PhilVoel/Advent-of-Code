input=: -&48 a. i. 'm' fread 'inputs/13.txt'
no_empty=: (* +/"1 input+16) # input
get_relevant=: monad define
	y #~ (y=_4) + (10>y) * y>:0
)
parse_num=: monad define
	(%&10) 0]F..{{10*y+x}} y
)
get_nums=: monad define
	split_at=: y i. -4
	|: (parse_num split_at{.y) ,. parse_num}.split_at}.y
)
process_machine=: dyad define
	a=: get_nums get_relevant {.y
	b=: get_nums get_relevant {.}.y
	p=: x + get_nums get_relevant {.2}.y
	solutions=: p%.a,.b
	btn_presses=: (*/"1 (=<.) solutions) # solutions
	({.}.btn_presses) + 3*{.btn_presses
)
part1=: +/ (1 0 0 $~ #no_empty) 0&process_machine;.1 no_empty
part2=: 15": +/ (1 0 0 $~ #no_empty) 10000000000000&process_machine;.1 no_empty
