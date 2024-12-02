module Solution where
    import Data.Char (digitToInt)
    import Data.List (sort)

    type CommonIn = ([Integer], [Integer])
    type CommonOut = [Integer]

    commonPre input = unzip $ map ((\[l,r] -> (read l, read r)) . words) input

    part1 (left, right) = zipWith ((abs .) . (-)) (sort left) (sort right)

    part2 (left, right) = map (\l -> sum $ filter (==l) right) left

    commonPost l = sum l
