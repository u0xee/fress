// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

extern crate fress;

use fress::Value;
use fress::memory::{schedule, segment};
use fress::vector::harness::{self, Op, Ops};

use std::panic;
use std::thread;
use fress::map::BITS;
use fress::map::MASK;
use fress::map::MAX_LEVELS;

fn format_hash_digits(mut hash: u32) -> String {
    let digits = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut s = String::new();
    for _ in 0..MAX_LEVELS {
        let x = hash & MASK;
        hash = hash >> BITS;
        s.push(digits[x as usize] as char);
    }
    s
}
fn equal_digits(s: &String, t: &String, digits: usize) -> bool {
    s.as_bytes()[..digits] == t.as_bytes()[..digits]
}
fn unique_in_digits(v: &[String], digits: usize) -> u32 {
    let mut count = 0;
    if !equal_digits(&v[0], &v[1], digits) {
        count += 1;
    }
    if !equal_digits(&v[v.len() - 1], &v[v.len() - 2], digits) {
        count += 1;
    }
    for i in 1..(v.len() - 1) {
        if !equal_digits(&v[i], &v[i - 1], digits) &&
           !equal_digits(&v[i], &v[i + 1], digits) {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut m = fress::hash_map();
    for i in 0..26 {
        let c = (b'a' + i) as char;
        let v = Value::from(c);
        //format!("{} => {:08X}", v, v.hash());
        m = m.assoc(v, i.into());
    }
    //println!("{}", m);
    use std::env;
    use std::fs;
    let args: Vec<String> = env::args().collect();
    let s = if args.len() < 2 { String::from(WORDS) } else {
        fs::read_to_string(&args[1]).unwrap()
    };

    let mut v: Vec<String> = s.split_whitespace().map(|w| format!("{}",w)).collect();
    let mut set = fress::hash_set();
    for a in &v {
        set = set.conj(Value::from(&a[..]));
    }
    println!("{}", set);
    for a in &v {
        set = set.dissoc(&Value::from(&a[..]));
    }
    println!("{}", set);
    if set.count() < 5 {
        return;
    }
    let mut v: Vec<String> = s.split_whitespace().map(|w| {
        format!("{} {}", format_hash_digits(Value::from(w).hash()), w)
    }).collect();
    v.sort();
    if v.len() < 600 {
        for a in &v {
            println!("{}", a);
        }
    } else {
        for a in &v[..200] {
            println!("{}", a);
        }
        for a in &v[(v.len() - 200)..] {
            println!("{}", a);
        }
    }
    println!("Total: {}, BITS: {}", v.len(), BITS);
    let mut cum = 0;
    for digits in 2..7 {
        let uniq = unique_in_digits(&v, digits);
        let cum_uniq = 100. * uniq as f64 / v.len() as f64;
        let exact_uniq = 100. * (uniq - cum) as f64 / v.len() as f64;
        println!("Unique in {}: {:3}/{} {:4.1}%  {:4.1}%",
                 digits, uniq - cum, v.len(), exact_uniq, cum_uniq);
        cum = uniq;
    }

    let mut cum = 0.0;
    for digits in 2..7 {
        let mut w: Vec<String> = v.iter().map(|s| s[..digits].to_string()).collect();
        w.dedup();
        let total = 1 << (digits * BITS as usize);
        let per = 100. - (100. * w.len() as f64 / total as f64);
        println!("Used patterns in {}: {:3}/{:8} {:4.1}%  {:4.1}%",
                 digits, w.len(), total, per - cum, per);
        cum = per;
    }

    /*
    let mut s: Vec<String> = (0..26).map(|i| Value::from((b'a' + i) as char))
        .map(|v| format!("{} => {:08X}", v, v.hash())).collect();
    s.sort_by_key(|a| *(a.as_bytes().last().unwrap()));
    for a in &s {
        println!("{}", a);
    }
    */

    /*
hash(\u) => B2B695F0
hash(\b) => 8C03AD51
hash(\g) => C96BFD42
hash(\n) => AA7A6DD2
hash(\i) => 95394A73
hash(\f) => BFCFF855
hash(\k) => D5CD9736
hash(\m) => D88D97F6
hash(\t) => DD5F1568
hash(\v) => 8B0A03D8
hash(\x) => A05DEAF8
hash(\c) => F6501FF9
hash(\e) => DFB5CD69
hash(\w) => A9ABE76B
hash(\d) => CA012E0B
hash(\p) => 8EF31D3B
hash(\q) => ECF1C06B
hash(\h) => D420C96C
hash(\j) => A90C7C8D
hash(\r) => AB5DC7CD
hash(\y) => 94AF2C7D
hash(\z) => E12AA7ED
hash(\a) => D500851E
hash(\s) => B574B50F
hash(\o) => ABAB269F
hash(\l) => EF71637F
{\u 20, \b 1, \i 8, \f 5, \h 7, \a 0, \g 6, \n 13, \k 10, \m 12, \t 19, \v 21, \x 23, \e 4, \c 2, \d 3, \p 15, \q 16, \w 22, \y 24, \j 9, \r 17, \z 25, \s 18, \l 11, \o 14}
    let ops3 = Ops(vec![Op::New(289)],
                   vec![Op::Conj(7), Op::Conj(4), Op::Set{index: 5, elem: 11}],
                   vec![Op::Conj(9), Op::Set{index: 5, elem: 22}]);
    let ops = Ops(vec![Op::New(288)], vec![Op::Conj(7)], vec![Op::Conj(9)]);
    let ops1 = Ops(vec![Op::New(7)], vec![Op::Conj(7)], vec![Op::Conj(9)]);
    let ops2 = Ops(vec![Op::New(17)], vec![Op::Conj(7)], vec![Op::Conj(9)]);
    harness::explore_schedules(ops3, 3);
    */
    /*
    let r = panic::catch_unwind(|| {
        println!("Ready");
        panic!("Go");
        42
    });
    println!("Steady");
    //panic::resume_unwind(r.unwrap_err());
    let x = r.unwrap_err().downcast::<&str>().unwrap();
    println!("{:?}", x);
    */
}

static WORDS: &str = "down
downbeat
downbeats
downcast
downed
downer
downers
downfall
downfalls
downgrade
downgraded
downgrades
downgrading
downhill
downier
downiest
downing
download
downloaded
downloading
downloads
downpatrick
downplay
downplayed
downplaying
downplays
downpour
downpours
downrange
downright
downs
downside
downsides
downstairs
downstream
downswing
downswings
downtime
downtimes
downtown
downtrend
downtrends
downtrodden
downturn
downturns
downward
downwards
downwind
downy
dowress
dowresses
dowries
dowry
dowse
dowsed
dowser
dowsers
dowses
dowsing
doxologies
doxology
doyen
doyle
doyley
doze
dozed
dozen
dozens
dozes
dozier
doziest
dozing
dozy
dr
drab
drabber
drabbest
drably
drabness
drabs
dracaena
dracaenas
drachm
drachma
drachmae
drachmas
draco
draconian
dracula
draff
draft
drafted
draftee
draftees
drafter
drafters
drafting
drafts
draftsman
draftsmen
draftsperson
drag
dragged
dragger
draggers
dragging
draggle
draggled
draggles
draggling
dragnet
dragnets
dragon
dragonflies
dragonfly
dragons
dragoon
dragooned
dragooning
dragoons
drags
dragster
dragsters
drain
drainage
drained
draining
drains
drake
drakes
dram
drama
dramamine
dramas
dramatic
dramatically
dramatics
dramatis
dramatisation
dramatisations
dramatise
dramatised
dramatises
dramatising
dramatist
dramatists
dramatization
dramatizations
dramatize
dramatized
dramatizes
dramatizing
dramaturgy
drambuie
drams
drank
drape
draped
draper
draperies
drapers
drapery
drapes
draping
drastic
drastically
drat
draught
draughtboard
draughtboards
draughtier
draughtiest
draughtiness
draughts
draughtsman
draughtsmanship
draughtsmen
draughty
draw
drawback
drawbacks
drawbridge
drawbridges
drawee
drawees
drawer
drawers
drawing
drawings
drawknife
drawl
drawled
drawling
drawls
drawn
draws
dray
drayage
drays
drayton
dread
dreaded
dreadful
dreadfully
dreading
dreadlocks
dreadnought
dreadnoughts
dreads
dream
dreamboat
dreamed
dreamer
dreamers
dreamier
dreamiest
dreamily
dreaminess
dreaming
dreamland
dreamlands
dreamless
dreamlessly
dreamlike
dreams
dreamt
dreamy
drearier
dreariest
drearily
dreariness
dreary
dred
dredge
dredged
dredger
dredgers
dredges
dredging
dregs
drench
drenched
drenches
drenching
drenges
dresden
dress
dressage
dressed
dresser
dressers
dresses
dressier
dressiest
dressiness
dressing
dressings
dressmaker
dressmakers
dressmaking
dressy
drew
drib
dribble
dribbled
dribbles
dribbling
driblet
driblets
dribs
dried
drier
driers
dries
driest
drift
drifted
drifter
drifters
drifting
drifts
driftwood
drill
drilled
drilling
drillings
drills
drily
drink
drinkable
drinker
drinkers
drinking
drinks
drip
dripped
dripping
drippings
drippy
drips
drive
drivel
driveled
driveling
drivelled
drivelling
drivels
driven
driver
drivers
drives
driveway
driveways
driving
drizzle
drizzled
drizzles
drizzling
drizzly
drogheda
drogue
drogues
droit
droits
droitwich
droll
droller
drolleries
drollery
drollest
drollness
drolly
dromedaries
dromedary
drone
droned
drones
droning
drool
drooled
drooling
drools
droop
drooped
droopier
droopiest
droopily
droopiness
drooping
droops
droopy
drop
droplet
droplets
dropout
dropouts
dropped
dropper
droppers
dropping
droppings
drops
dropsy
dross
drought
droughts
drove
drover
drovers
droves
drown
drowned
drowning
drowns
drowse
drowsed
drowses
drowsier
drowsiest
drowsily
drowsiness
drowsing
drowsy
drub
drubbed
drubbing
drubs
drudge
drudged
drudgeries
drudgery
drudges
drudging
drug
drugged
drugging
druggist
druggists
drugs
drugstore
drugstores
druid
druids
drum
drumbeat
drumbeats
drumhead
drumheads
drumlin
drumlins
drummed
drummer
drummers
drumming
drummond
drums
drumstick
drumsticks
drunk
drunkard
drunkards
drunken
drunkenly
drunkenness
drunker
drunkest
drunks
drupaceous
drupe
drupes
drury
dry
dryad
dryads
dryden
dryer
dryers
drying
dryings
dryly
dryness
dsc
dsm
dso
dsos
du
dual
dualism
dualistic
dualistically
duality
dually
duarchies
duarchy
dub
dubai
dubbed
dubbin
dubbing
dubiety
dubiosity
dubious
dubiously
dubiousness
dubitante
dubitantes
dubitatur
dubitavit
dublin
dubliner
dubrovnik
dubs
ducal
ducat
ducats
duces
duchess
duchesses
duchies
duchy
duck
duckbill
duckbills
ducked
ducking
duckling
ducklings
ducks
duckweed
ducky
duct
ductile
ductility
ductless
ducts
ductwork
dud
dude
dudes
dudgeon";

