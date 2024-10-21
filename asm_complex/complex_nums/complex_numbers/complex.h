
struct MyComplex
{
    float real;
    float imaginaer;
};

void init();

struct MyComplex add(struct MyComplex op1, struct MyComplex op2);

struct MyComplex sub(struct MyComplex op1, struct MyComplex op2);

struct MyComplex div(struct MyComplex zeahler, struct MyComplex nenner);

struct MyComplex multiply(struct MyComplex op1, struct MyComplex op2);