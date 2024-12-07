import System.Environment (getArgs)

append l r = r + l * (10 ^ (1 + (floor . logBase 10 . fromIntegral) r))

isPossible _ target current [] = target == current
isPossible part2 target current (h:rest) = target >= current && (isPossible part2 target (current+h) rest || isPossible part2 target (current*h) rest || part2 && isPossible part2 target (append current h) rest)

main = do
    args <- getArgs
    input <- readFile "inputs/7.txt"
    print . sum . map (\(x,_,_) -> x) . filter (\(h,f,t) -> isPossible ("--part2" `elem` args) h f t) . map ((\(h:f:t) -> (read $ init h, read f, map read t)) . words) . lines $ input
