import System.Environment (getArgs)

commonPre i = map (map read . words) $ lines i

between x y z = x <= z && y >= z
isSafe l = all (between 1 3) l || all (between (-3) (-1)) l
getDiffs = map (\(f:r) -> fst $ foldl (\(acc, last) curr -> ((last - curr) : acc, curr)) ([], f) r)
getAllSublists l = l : map (\i -> let (x,y) = splitAt i l in x ++ tail y) [0..length l-1]

part1 = filter isSafe . getDiffs

part2 = filter (any isSafe) . map (getDiffs . getAllSublists)

main = do
    args <- getArgs
    input <- readFile "inputs/2.txt"
    print $
        if "--part2" `notElem` args then
            length . part1 . commonPre $ input
        else
            length . part2 . commonPre $ input
