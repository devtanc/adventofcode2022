       IDENTIFICATION DIVISION.
       PROGRAM-ID. ADVENT-OF-CODE-2022-DAY-1.
       AUTHOR. DEVTANC.
      
       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT ELVES ASSIGN TO 'input.txt'
           ORGANIZATION IS LINE SEQUENTIAL.
       
       DATA DIVISION. 
       FILE SECTION.
       FD ELVES.
       01 ELVES-FILE.
           05 CALORIES     PIC A(5).

       WORKING-STORAGE SECTION. 
       01 WS-ELVES.
           05 WS-CALORIES  PIC 9(6).
       01 WS-CALORIE-TOTALS-TABLE.
           05 WS-TOTAL PIC 9(7) OCCURS 300 TIMES INDEXED BY I.
       01 WS-EOF              PIC A(1).
       01 WS-CALORIE-TOTAL    PIC 9(6) VALUE 0.
       01 WS-CALORIE-JUST     PIC X(6) JUSTIFIED RIGHT.
       01 WS-CALORIE-NUM      PIC 9(5) VALUE 0.
       01 HIGHEST-CALORIE     PIC 9(7) VALUE 0.
       01 THREE-HIGHEST-CALORIE     PIC 9(8) VALUE 0.
       
       PROCEDURE DIVISION.           
           OPEN INPUT ELVES.
           SET I TO 1.
           
           PERFORM UNTIL WS-EOF = 'Y'
             READ ELVES INTO WS-ELVES
             AT END
      *        DISPLAY "RUNNING TOTAL: " WS-CALORIE-TOTAL
               MOVE WS-CALORIE-TOTAL TO WS-TOTAL(I)
               ADD 1 TO I
               MOVE 'Y' TO WS-EOF
             NOT AT END
               IF WS-CALORIES = SPACE THEN
      *          DISPLAY "RUNNING TOTAL: " WS-CALORIE-TOTAL
                 MOVE WS-CALORIE-TOTAL TO WS-TOTAL(I)
                 ADD 1 TO I
                 SET WS-CALORIE-TOTAL TO 0
                 CONTINUE
               END-IF
               IF WS-CALORIES = LOW-VALUE THEN
      *          DISPLAY "RUNNING TOTAL: " WS-CALORIE-TOTAL
                 MOVE WS-CALORIE-TOTAL TO WS-TOTAL(I)
                 ADD 1 TO I
                 SET WS-CALORIE-TOTAL TO 0
                 CONTINUE
               END-IF
               UNSTRING WS-CALORIES DELIMITED BY SPACE
                 INTO WS-CALORIE-JUST
               INSPECT WS-CALORIE-JUST REPLACING ALL SPACES BY ZEROES
               MOVE WS-CALORIE-JUST TO WS-CALORIE-NUM
               ADD WS-CALORIE-NUM TO WS-CALORIE-TOTAL
             END-READ
           END-PERFORM.
           CLOSE ELVES.
           
           SORT WS-TOTAL DESCENDING WS-TOTAL
           
           MOVE WS-TOTAL(1) TO HIGHEST-CALORIE
           ADD WS-TOTAL(1) WS-TOTAL(2) WS-TOTAL(3)
              TO THREE-HIGHEST-CALORIE 
           
           DISPLAY "HIGHEST: " HIGHEST-CALORIE 
           DISPLAY "THREE-H: " THREE-HIGHEST-CALORIE 
           STOP RUN.
           