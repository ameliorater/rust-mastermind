# rust-mastermind
This program allows you to play 6x10 Mastermind against the computer. You will be the code maker, and the computer will be the code breaker. Your objective is to come up with a hard-to-guess code, and the computer's objective is to guess it in as few guesses as possible.

<br>

__Instructions:__
- Please use ``cargo run --release``
- Enter code for automatic mode or press enter on first prompt for manual response mode

<br>

__How to play Mastermind (6x10 version):__
- Come up with a six-digit code using any digits 0-9 (repetitions allowed)
- To form a response, count how many digits from the guessed code are correct and in the right place, and how many are correct but in the wrong place
- Enter your response in the form ``RightPlaceWrongPlace`` (ex: ``60`` for all guess exactly matching code)

<br>

__Response Examples:__

``Code : 123456``
 <br>
``Guess: 126321``
<br>
``Response -> 22``

<br>

``Code :  112233``
<br>
``Guess: 123456``
<br>
``Response -> 12``

<br>

``Code : 345666``
<br>
``Guess: 634566``
<br>
``Response -> 24``
