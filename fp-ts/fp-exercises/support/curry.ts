// Slightly limited implementation of a general curry function
// Found here: https://stackoverflow.com/a/61171266/18573230
// Takes in any function, but you have to pass in arguments one-by-one
//
// More general solution is here: https://github.com/millsp/ts-toolbelt/blob/master/sources/Function/Curry.ts#L99

// Given a function, returns the type of the first argument
type CurryFirst<T> = T extends (x: infer U, ...rest: any) => any ? U : never;
// Returns type of curried function with first argument applied
type CurryRest<T> = 
    T extends (x: infer U) => infer V ? U :
    T extends (x: infer U, ...rest: infer V) => infer W ? Curried<(...args: V) => W> : 
    never;

type Curried<T extends (...args: any) => any> = (x: CurryFirst<T>) => CurryRest<T>;


const curry = <T extends (...args: any) => any> (fn: T): Curried<T> => {
    if (!fn.length) { return fn(); }
    return (arg: CurryFirst<T>): CurryRest<T> =>  {
        return curry(fn.bind(null, arg) as any) as any;
    }
} 

export default curry;
