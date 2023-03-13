/*


    [] (array access)

    The array access operator is used to specify a location within an array
    = (assign)

    Assigns a value to a variable
    catch

    The catch keyword is used with try to handle exceptions
    class

    Keyword used to indicate the declaration of a class
    , (comma)

    Separates parameters in function calls and elements during assignment
    // (comment)

    Explanatory notes embedded within the code
    {} (curly braces)

    Define the beginning and end of functions blocks and statement blocks such as the for and if structures
    /** */ (doc comment)

    Explanatory notes embedded within the code
    . (dot)

    Provides access to an object's methods and data
    draw()

    Called directly after setup() and continuously executes the lines of code contained inside its block until the program is stopped or noLoop() is called
    exit()

    Quits/stops/exits the program
    extends

    Allows a new class to inherit the methods and data fields (variables and constants) from an existing class
    false

    Reserved word representing the logical value "false"
    final

    Keyword used to state that a value, class, or method can't be changed
    implements

    Implements an interface or group of interfaces
    import

    The keyword import is used to load a library into a Processing sketch
    loop()

    Causes Processing to continuously execute the code within draw()
    /* */ (multiline comment)

    Explanatory notes embedded within the code
    new

    Creates a "new" object
    noLoop()

    Stops Processing from continuously executing the code within draw()
    null

    Special value used to signify the target is not a valid data element
    () (parentheses)

    Grouping and containing expressions and parameters
    popStyle()

    Saves the current style settings and popStyle() restores the prior settings
    pop()

    The pop() function restores the previous drawing style settings and transformations after push() has changed them
    private

    This keyword is used to disallow other classes access to the fields and methods within a class
    public

    Keyword used to provide other classes access the fields and methods within a class
    pushStyle()

    Saves the current style settings and popStyle() restores the prior settings
    push()

    The push() function saves the current drawing style settings and transformations, while pop() restores these settings
    redraw()

    Executes the code within draw() one time
    return

    Keyword used to indicate the value to return from a function
    ; (semicolon)

    A statement terminator which separates elements of the program
    setLocation()

    The setLocation() function defines the position of the Processing sketch in relation to the upper-left corner of the computer screen
    setResizable()

    By default, Processing sketches can't be resized
    setTitle()

    The setTitle() function defines the title to appear at the top of the sketch window
    setup()

    The setup() function is called once when the program starts
    static

    Keyword used to define a variable as a "class variable" and a method as a "class method
    super

    Keyword used to reference the superclass of a subclass
    this

    Refers to the current object (i
    thread()

    Launch a new thread and call the specified function from that new thread
    true

    Reserved word representing the logical value "true"
    try

    The try keyword is used with catch to handle exceptions
    void

    Keyword used to indicate that a function returns no value


 */

// [] (array access)

//= (assign)

// catch

// class

// , comma

// (comment)

// {} (curly braces)

// /** */ (doc comment)

// . (dot)

// draw()

// exit()

// extends

// false

// final

// implements

// import

// loop()

// /* */ (multiline comment)

// new

// noLoop()

// null

// () (parentheses)

// popStyle()

pub fn pop(engine: &mut Engine) {
//    engine.param.matriz_traslacion = engine.parambakup.matriz_traslacion.clone();
//    engine.param.matriz_rotacionx = engine.parambakup.matriz_rotacionx.clone();
//    engine.param.matriz_rotaciony = engine.parambakup.matriz_rotaciony.clone();
//    engine.param.matriz_rotacionxyz = engine.parambakup.matriz_rotacionxyz.clone();
    pop_matrix(&mut engine.param);

    engine.param.fill_color = engine.parambakup.fill_color;
    engine.param.fill_bool = engine.parambakup.fill_bool;
    engine.param.stroke_bool = engine.parambakup.stroke_bool;
    engine.param.stroke_color = engine.parambakup.stroke_color;
    engine.param.stroke_weight = engine.parambakup.stroke_weight;
    engine.param.colormode = engine.parambakup.colormode.clone();
    engine.param.rect_mode = engine.parambakup.rect_mode.clone();
}

// private

// public

// pushStyle()

// Al hacer push y pop se controlan las siguientes funciones
// Ojo está incompleto
// fill(), stroke(), tint(), strokeWeight(), strokeCap(), strokeJoin(),
// imageMode(), rectMode(), ellipseMode(), colorMode(), textAlign(),
// textFont(), textMode(), textSize(), textLeading().

use crate::engine::Engine;
use crate::transform::{pop_matrix, push_matrix};

pub fn push(engine: &mut Engine) {
//    engine.parambakup.matriz_traslacion = engine.param.matriz_traslacion.clone();
//    engine.parambakup.matriz_rotacionx = engine.param.matriz_rotacionx.clone();
//    engine.parambakup.matriz_rotaciony = engine.param.matriz_rotaciony.clone();
//    engine.parambakup.matriz_rotacionxyz = engine.param.matriz_rotacionxyz.clone();
    push_matrix(&mut engine.param);

    engine.parambakup.fill_color = engine.param.fill_color;
    engine.parambakup.fill_bool = engine.param.fill_bool;
    engine.parambakup.stroke_bool = engine.param.stroke_bool;
    engine.parambakup.stroke_color = engine.param.stroke_color;
    engine.parambakup.stroke_weight = engine.param.stroke_weight;
    engine.parambakup.colormode = engine.param.colormode.clone();
    engine.parambakup.rect_mode = engine.param.rect_mode.clone();
}

// redraw()

// return

// ; (semicolon)

// setLocation()

// setResizable()

// setTitle()

// setup()

// static

// super

//    this

//    thread()

//    true

//    try

//    void

