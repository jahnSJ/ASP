#include "complex.h"

/*
As a reference for the Constraints, used in the following asm functions:
https://gcc.gnu.org/onlinedocs/gcc/Machine-Constraints.html

ARM FPU ISA:
https://developer.arm.com/documentation/ddi0439/b/Floating-Point-Unit/FPU-Functional-Description/FPU-instruction-set

In addition, the following code has been inspired by:
https://docs.rs/num-complex/latest/src/num_complex/lib.rs.html#811
*/

void init(){
    asm volatile inline (
      "LDR.W R0, =0xE000ED88\n\t"
           "LDR R1, [R0]\n\t"
           "ORR R1, R1, #(0xF << 20)\n\t"
           "STR R1, [R0]\n\t"
           "DSB\n\t"
           "ISB"
      );
}

struct MyComplex add(struct MyComplex op1, struct MyComplex op2) {
    struct MyComplex out;

      asm volatile inline (
          "vadd.F32 %[out],%[op1],%[op2]"
          : [out] "=t" (out.real)
          : [op1] "t" (op1.real),
            [op2] "t" (op2.real)
      );

      asm volatile inline (
          "vadd.F32 %[out],%[op1],%[op2]"
          : [out] "=t" (out.imaginaer)
          : [op1] "t" (op1.imaginaer),
            [op2] "t" (op2.imaginaer)
        );

    return out;
}

struct MyComplex sub(struct MyComplex op1, struct MyComplex op2) {
    struct MyComplex out;

    asm volatile inline (
      "vsub.F32 %[out],%[op1],%[op2]"
          : [out] "=t" (out.real)
          : [op1] "t" (op1.real),
            [op2] "t" (op2.real)
        );

    asm volatile inline (
      "vsub.F32 %[out],%[op1],%[op2]"
          : [out] "=t" (out.imaginaer)
          : [op1] "t" (op1.imaginaer),
            [op2] "t" (op2.imaginaer)
        );

    return out;
}


struct MyComplex div(struct MyComplex zeahler, struct MyComplex nenner) {
    struct MyComplex out;

    float platzhalter1;
    float platzhalter2;

    asm volatile inline (
        "vmul.F32 %[product1],%[op3],%[op3]\n\t"
        "vmul.F32 %[product2],%[op4],%[op4]"
        : [product1] "=t" (platzhalter1),
          [product2] "=t" (platzhalter2)
        : [op1] "t" (zeahler.real),
          [op2] "t" (zeahler.imaginaer),
          [op3] "t" (nenner.real),
          [op4] "t" (nenner.imaginaer)
    );

    float divisor;

   asm volatile inline (
      "vadd.F32 %[div],%[temporal1],%[temporal2]"
       : [div] "=t" (divisor)
       : [temporal1] "t" (platzhalter1),
         [temporal2] "t" (platzhalter2)
    );

    float platzhalter3;
    float platzhalter4;

    asm volatile inline (
       "vmul.F32 %[product3],%[op1],%[op3]\n\t"
       "vmul.F32 %[product4],%[op2],%[op4]"
        : [product3] "=t" (platzhalter3),
          [product4] "=t" (platzhalter4)
        : [op1] "t" (zeahler.real),
          [op2] "t" (zeahler.imaginaer),
          [op3] "t" (nenner.real),
          [op4] "t" (nenner.imaginaer)
    );

    float temp1;

    asm volatile inline (
        "vadd.F32 %[result1],%[temporal3],%[temporal4]"
         : [result1] "=t" (temp1)
         : [temporal3] "t" (platzhalter3),
           [temporal4] "t" (platzhalter4)
    );

    asm volatile inline (
        "vdiv.F32 %[real],%[zaehler1],%[divider]"
         : [real] "=t" (out.real)
         : [zaehler1] "t" (temp1),
           [divider] "t" (divisor)
     );

    float platzhalter5;
    float platzhalter6;

    asm volatile inline (
        "vmul.F32 %[product5],%[op2],%[op3]\n\t"
        "vmul.F32 %[product6],%[op1],%[op4]"
         : [product5] "=t" (platzhalter5),
           [product6] "=t" (platzhalter6)
         : [op1] "t" (zeahler.real),
           [op2] "t" (zeahler.imaginaer),
           [op3] "t" (nenner.real),
           [op4] "t" (nenner.imaginaer)
    );

    float temp2;

    asm volatile inline (
      "vsub.F32 %[result2],%[temporal5],%[temporal6]"
       : [result2] "=t" (temp2)
       : [temporal5] "t" (platzhalter5),
         [temporal6] "t" (platzhalter6)
    );

    float platzhalter7;
    float platzhalter8;

    asm volatile inline (
        "vmul.F32 %[product1],%[op3],%[op3]\n\t"
        "vmul.F32 %[product2],%[op4],%[op4]"
         : [product1] "=t" (platzhalter7),
           [product2] "=t" (platzhalter8)
         : [op1] "t" (zeahler.real),
           [op2] "t" (zeahler.imaginaer),
           [op3] "t" (nenner.real),
           [op4] "t" (nenner.imaginaer)
    );

    float divider;

    asm volatile inline (
        "vadd.F32 %[div],%[temporal1],%[temporal2]"
        : [div] "=t" (divider)
        : [temporal1] "t" (platzhalter1),
          [temporal2] "t" (platzhalter2)
    );

    asm volatile inline (
        "vdiv.F32 %[im],%[zaehler2],%[divi]"
        : [im] "=t" (out.imaginaer)
        : [zaehler2] "t" (temp2),
          [divi] "t" (divider)
    );

    return out;
}


struct MyComplex multiply(struct MyComplex op1, struct MyComplex op2) {
    struct MyComplex out;

    float rd1;
    float rd2;

    asm volatile inline (
      "vmul.F32 %[rd1],%[op1],%[op2]\n\t"
      "vmul.F32 %[rd2],%[op3],%[op4]"
      : [rd1] "=t" (rd1),
        [rd2] "=t" (rd2)
      : [op1] "t" (op1.real),
        [op2] "t" (op2.real),
        [op3] "t" (op1.imaginaer),
        [op4] "t" (op2.imaginaer)
           );

      asm volatile inline (
        "vsub.F32 %[rd],%[rd1],%[rd2]"
        : [rd] "=t" (out.real)
        : [rd1] "t" (rd1),
          [rd2] "t" (rd2)
        );

      float rd3;
      float rd4;

      asm volatile inline (
        "vmul.F32 %[rd1],%[op1],%[op2]\n\t"
        "vmul.F32 %[rd2],%[op3],%[op4]"
        : [rd1] "=t" (rd3),
          [rd2] "=t" (rd4)
        : [op1] "t" (op1.real),
          [op2] "t" (op2.imaginaer),
          [op3] "t" (op1.imaginaer),
          [op4] "t" (op2.real)
        );

     asm volatile inline (
       "vadd.F32 %[rd],%[rd1],%[rd2]"
        : [rd] "=t" (out.imaginaer)
        : [rd1] "t" (rd3),
          [rd2] "t" (rd4)
        );

    return out;
}
