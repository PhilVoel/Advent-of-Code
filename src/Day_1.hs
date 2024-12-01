import Data.Char (digitToInt)
import Data.List (sort)
import System.Environment (getArgs)

main :: IO ()
main = do
    args <- getArgs
    input <- readFile "inputs/1.txt"
    let (left, right) = unzip . map ((\[l,r] -> (read l, read r)) . words) $ lines input
    print . sum $
        if "--part2" `notElem` args then
            zipWith ((abs .) . (-)) (sort left) (sort right)
        else
            map (\l -> sum $ filter (==l) right) left
