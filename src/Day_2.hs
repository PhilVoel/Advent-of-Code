module Solution where
    commonPre :: [String] -> [[Integer]]
    commonPre = map (map read . words)

    between x y z = x <= z && y >= z
    isSafe l = all (between 1 3) l || all (between (-3) (-1)) l
    getDiffs = map (\(f:r) -> fst $ foldl (\(acc, last) curr -> ((last - curr) : acc, curr)) ([], f) r)
    getAllSublists l = l : map (\i -> let (x,y) = splitAt i l in x ++ tail y) [0..length l-1]

    part1 = filter isSafe . getDiffs

    part2 = filter (any isSafe) . map (getDiffs . getAllSublists)

    commonPost :: [a] -> Int
    commonPost = length
