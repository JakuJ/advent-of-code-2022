(declare-const beacon_x Int)
(declare-const beacon_y Int)

(define-fun manhattan_distance ((x1 Int) (y1 Int) (x2 Int) (y2 Int)) Int
    (+ (abs (- x1 x2)) (abs (- y1 y2))))

(define-fun sensor ((sx Int) (sy Int) (bx Int) (by Int)) Bool
    (< (manhattan_distance sx sy bx by) (manhattan_distance sx sy beacon_x beacon_y)))

; bounds check
(assert (<= 0 beacon_x))
(assert (<= 0 beacon_y))
(assert (>= 4000000 beacon_x))
(assert (>= 4000000 beacon_y))

; sensor data
(assert (sensor 3797530 3451192 3316341 3328308))
(assert (sensor 3779164 33938 4608350 708806))
(assert (sensor 1331810 3260896 2075597 3280016))
(assert (sensor 393374 696899 2021690 453306))
(assert (sensor 2928048 923094 2021690 453306))
(assert (sensor 2386726 3645023 2075597 3280016))
(assert (sensor 1900159 2381031 1649961 2000000))
(assert (sensor 2601378 2979844 2218962 2701963))
(assert (sensor 2254818 32199 2021690 453306))
(assert (sensor 2689643 375840 2021690 453306))
(assert (sensor 909141 2842547 2218962 2701963))
(assert (sensor 3915731 2454320 4268501 1853073))
(assert (sensor 1693574 1344104 1649961 2000000))
(assert (sensor 1760260 3297662 2075597 3280016))
(assert (sensor 1909567 3990737 2075597 3280016))
(assert (sensor 2097863 3179766 2075597 3280016))
(assert (sensor 3100489 3623847 3104748 4102403))
(assert (sensor 2746023 2432826 2218962 2701963))
(assert (sensor 3031245 3031354 3316341 3328308))
(assert (sensor 277094 1999350 1649961 2000000))
(assert (sensor 1763269 126349 2021690 453306))
(assert (sensor 3287624 2695420 3316341 3328308))
(assert (sensor 2371102 1745103 1649961 2000000))
(assert (sensor 3553438 1563379 4268501 1853073))
(assert (sensor 1529129 2735122 2218962 2701963))
(assert (sensor 2826220 3958350 3104748 4102403))
(assert (sensor 3999334 3912693 3104748 4102403))
(assert (sensor 240430 3829436 (- 742036) 3963149))
(assert (sensor 3455748 3814861 3316341 3328308))

; get solution
(check-sat) ; sat
(get-model) ; beacon_x = 2829680, beacon_y = 3411840

; verify that no other solution exists
(assert (not (= beacon_x 2829680)))
(assert (not (= beacon_y 3411840)))
(check-sat) ; unsat
