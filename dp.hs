import Control.Monad.State
import Data.Array

coins = [1, 3, 20]

type Solved = Array Int (Maybe Int)

a :: Int -> Solved
a amax = listArray (0, amax) [Nothing | i <- [0..amax]]

solve :: Int -> State Solved Int
solve x | x < 1 = return 0
solve x = do
    solved <- get
    case solved ! x of
        Just s -> return s
        Nothing -> do
            solves <- mapM (\c -> solve $ x - c) coins
            solved <- get
            let result = minimum solves + 1
            put $ solved // [(x, Just result)]
            return result
