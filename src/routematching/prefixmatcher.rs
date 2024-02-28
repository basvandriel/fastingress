// this module should cover
//
// MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/example/hi
//
// MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/example/hi/bas
//
// MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/example/hi/bas/hi2
//
// NO MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/example/hi__shouldnotmatch
//
// NO MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/example/bas
//
// NO MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/bas
