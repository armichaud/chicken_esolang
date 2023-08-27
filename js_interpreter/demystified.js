// The original: https://web.archive.org/web/20180420010853/http://torso.me/chicken

// I still don't feel like I understand this entirely, but this was useful for debugging.
// Here's what the variables generally refer to during execution:

// chicken.Chicken and chicken.chicken => Stack
// CHICKEN                             => latest item popped off the stack
// Chicken                             => current instruction
// chicken.CHICKEN                     => Pointer to top of stack
// chicken.$Chicken                    => Instruction Pointer


function chicken(CHICKEN, Chicken) {
    // Code is not blank
    Chicken && (
        // Initialize Stack
        chicken.chicken = [
            // First is undefined
            ,
            // Second is the user input
            CHICKEN,
            // Initializes everything to 0, except chicken.Chicken, which is the code
            // This is because the comparison always evaluates to false, which is then cast as a number with the minus sign
            CHICKEN = Chicken = chicken.$Chicken = -(CHICKEN == (chicken.Chicken = Chicken))
        ],
        // Set register 0 as self reference
        // appended ++ increments, but returns the original value
        chicken.chicken[Chicken++] = chicken.chicken,
        // Sets chicken.CHICKEN to 2
        // prepended ++ increments and returns the incremented value
        chicken.CHICKEN = ++Chicken, 

        // decrements Chicken to 1, recursively calling Chicken with 1 as user input, no code
        chicken(--Chicken),
        // Sets chicken.$Chicken to 2
        chicken.$Chicken = ++Chicken, 
        // Sets chicken.CHICKEN to 3
        chicken.CHICKEN++ 
    );
    // Gets first instruction
    // Increments chicken.$Chicken to 3, and sets Chicken to chicken.Chicken[2]
    Chicken = chicken.Chicken[chicken.$Chicken++];

    // Sets final output
    chicken.Chicken = 
        CHICKEN? 
            // Parse the code
            Chicken?
                // Octal for newline feed
                '\012'== Chicken? 
                    chicken(++CHICKEN, chicken.chicken[++chicken.CHICKEN] = CHICKEN - CHICKEN)
                    // Chicken is space or carriage return
                    : Chicken == ' ' | '\015' == Chicken ||
                        // Op is "chicken" 
                        (Chicken) == "c" &  chicken. Chicken [chicken.  $Chicken++ ]== "h" &  chicken. Chicken [chicken.  $Chicken++ ]== "i" &  chicken. Chicken [chicken.  $Chicken++ ]== "c" &  chicken. Chicken [chicken.  $Chicken++ ]== "k" &  chicken. Chicken [chicken.  $Chicken++ ]== "e" &  chicken. Chicken [chicken.  $Chicken++ ]== "n" 
                        && ++chicken.chicken[chicken.CHICKEN]
                // Recursive
                ? chicken(CHICKEN)
                : ["Error on line "+CHICKEN+": expected 'chicken'", chicken.CHICKEN = CHICKEN++-CHICKEN]
            : chicken.chicken 
        
        :(CHICKEN = chicken.Chicken[chicken.CHICKEN],
            // NOT ORIGINAL: this is my own debug statement
            // console.log({ first_registers: chicken.chicken.slice(1,4), opcode: Chicken, data: chicken.Chicken.slice(111)}), 
            Chicken? (
                Chicken = --Chicken?
                    --Chicken? 
                        --Chicken? 
                            --Chicken? 
                                --Chicken?
                                    --Chicken? 
                                        --Chicken? 
                                            --Chicken? 
                                                --Chicken? 
                                                    chicken.CHICKEN++ && --Chicken // Push
                                                : '&#'+CHICKEN+';' // Char
                                            : chicken.Chicken[chicken.Chicken[--chicken.CHICKEN] && (chicken.$Chicken += CHICKEN), --chicken.CHICKEN] // Jump
                                        : chicken.Chicken[chicken.Chicken[CHICKEN] = chicken.Chicken[--chicken.CHICKEN], --chicken.CHICKEN] // Store
                                    : chicken.Chicken[chicken.Chicken[chicken.$Chicken++]][CHICKEN] // Load
                                : CHICKEN == chicken.Chicken[--chicken.CHICKEN] // Comparison
                            : CHICKEN * chicken.Chicken[--chicken.CHICKEN] // Multiplication
                        : chicken.Chicken[--chicken.CHICKEN] - CHICKEN && chicken.CHICKEN == 1 // Subtraction
                    : chicken.Chicken[--chicken.CHICKEN] + CHICKEN // Addition
                : chicken.CHICKEN ++ && "chicken", // Chicken
            chicken.Chicken[chicken.CHICKEN] = Chicken, chicken(),
            console.log({ first_registers: chicken.chicken.slice(1,4), opcode: Chicken, data: chicken.Chicken.slice(111)})) 
            : CHICKEN 
        );

    return chicken.Chicken
}

// 99 chickens program for testing
/*
console.log(chicken('asdf', `chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken
chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken chicken
chicken
chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken


chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken

chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken chicken
chicken chicken chicken
chicken chicken chicken chicken chicken chicken chicken chicken
`));
*/
