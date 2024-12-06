mb=: 100 100 $ 0
in=. -&48 a. i. 'm' fread 'inputs/5.txt'
empty=. (+/+&16 |:in) i.0
rules=: mb]F..{{y+ (1 (<({.x);{:x) } mb)}} |: ($~2,(2%~#)) ((10&*@(0&{)+1&{),10&*@(2&{)+3&{) (2&{.,3&}.) 5{. |: empty{.in
books=: }.empty}.in
nums=: monad define
	s=: (#~-.@:e.&_16 _4) y
	(0$0)]F..{{y,s ((10&*@{.)+{:)@(2&{.@}.~(2&*)) x}} i.2%~#s
)
matr=: monad define
	rules*.100}.(mb,mb)]F..{{(1(<x)}"1(100{.y)), (100}.y)+(100{.y)*.1(<x)}mb}} y
)
p1=: monad define
	nms=. nums y
	(nms{~(1-~#nms)%2)*-.*+/+/matr nms
)
part1=: 0]F..{{y+(p1 x)}} books
p2f=: books#~(0$0)]F..{{y,(*+/+/matr nums x)}} books
part2=: 0]F..{{y+(p1 x)}} p2f
