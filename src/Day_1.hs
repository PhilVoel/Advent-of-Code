import Data.Char (digitToInt)
import Data.List (sort)
import System.Environment (getArgs)

splitUp input = unzip $ map ((\[l,r] -> (read l, read r)) . words) $ lines input

part1 (left, right) = zipWith ((abs .) . (-)) (sort left) (sort right)

part2 (left, right) = map (\l -> sum $ filter (==l) right) left

main = do
    args <- getArgs
    input <- readFile "inputs/1.txt"
    print . sum $
        if "--part2" `notElem` args then
            part1 . splitUp $ input
        else
            part2 . splitUp $ input
