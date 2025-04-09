

/// Applied to two pair-encoded lists, determines if 2nd argument exists within the first
///
/// contains xs x ≡ λxs λx.not (is_nil (filter (eq x) xs))
///             
/// # Example
/// ```
/// use lambda_calculus::data::boolean::{fls, not, tru};
/// use lambda_calculus::data::list::pair::{filter, head, is_nil};
/// use lambda_calculus::data::num::church::eq;
/// use lambda_calculus::*;
///
/// let xs = vec![
///     0.into_church(),
///     2.into_church(),
///     3.into_church(),
/// ].into_pair_list();
///
/// assert_eq!(beta(
///     app!(
///         contains(),
///         0.into_church(),
///         xs.clone()
///         )
///     ), NOR, 0), tru());
/// assert_eq!(beta( app!(contains(), 0.into_church(), xs.clone()) ), NOR, 0), fls());
/// assert_eq!(
///     beta( app!(contains(), 1.into_church(), vec![].into_pair_list()) ), NOR, 0),
///     fls()
/// );
/// ```


## Combinators

#### Identity 

$$
I = \lambda x.x
$$

#### Konstant

$$
K = \lambda xy.x
$$

synonymous with true, this is also known as the **discarding** combinator

#### Substitution

$$
S = \lambda xyz.x\; z\; (y \; z)
$$

These three give rise to the SKI calculus which alone is Turing Complete. (TODO: cite "To Dissect a Mockingbird:" https://dkeenan.com/Lambda/index.htm)

#### Iota 

The universal combinator
$$
i = \lambda x.x\; S \; K
$$

#### Composition

$$
B = \lambda xyz.x\; (y \; z)
$$
#### Swapping

$$
B = \lambda xyz.x\; z\; y
$$

#### Duplication

$$
W = \lambda xy.x\; y\; y
$$

#### Self-application

$$
\omega = \lambda x.x\; x
$$

#### Divergence (fork bomb)

$$
\Omega = \omega \; \omega
$$

TODO: then add stuff about Y being lazy, and TPFC "It is suitable for `NOR` (normal), `HNO` (hybrid normal), `CBN` (call-by-name), and `HSP` (head

/// spine) reduction `Order`s."

#### Strict Fixed Point

/// It will work with all the reduction orders suitable for its lazy counterpart (the `Y`

/// combinator). In addition, it will also work with `CBV` (call-by-value) and `HAP` (hybrid

/// applicative) reduction `Order`s, though it is not a drop-in replacement - in order for such

/// expressions to work, they need to be modified so that the evaluation of arguments of conditionals

/// and other terms that need to be lazy is delayed.

$$
Z = λf.\big(λx.f \; (λv.x \;x \;v)\big) \;\big(λx.f \; (λv.x \;x \;v)\big)
$$

#### Thrush

Reverse application

$$
R = \lambda xf.f\;x
$$


## Reduction Order

Whether or not our reduction traverses one of these intermediate states is determined by the β-reduction strategy we employ.  There are a number of well-researched reduction strategies which lend themselves to different types of expressions.  

Thus far, we've mainly been using **normal order** reduction, which reduces the leftmost, outermost β-redex first before proceeding to the sub-expressions contained within, or neighboring the left-most expression.   This effectively results in deferring the evaluation of the arguments to a function until the last possible moment.  

The obvious counterpart to normal order reduction is called **applicative order** which prioritizes the leftmost _innermost_ expression.  Applicative order is sometimes referred to a _eager_ evaluation, which may be unfit for unrolling some expressions as we saw with our $fac$ example.  

Consider the following expression under the two reduction orders:

$$
\lambda x.\big((x \; y ) \; (y \; x )\big) \; \big(\lambda w.(w \; w )\; z))
$$

Under normal order ($N$) reduction the abstraction binding $w$ will replace both occurrences of $x$ in the $(x \;y) \; (y \;x)$ applications in the first abstraction before evaluating the $\lambda w$ expressions:

$$
\begin{aligned}
	\lambda x.\big((x \; y ) \; (y \; x )\big) \; \big(\lambda w.(w \; w )\; z)) &\rightarrow_N \Big(\big(\big(\lambda w.(w \; w )\; z)) \; y ) \; (y \; \big(\lambda w.(w \; w )\; z)) \big)\Big) \\
	&\rightarrow_N \Big(\big((z\; z) \; y )  \; (y \; \big(\lambda w.(w \; w )\; z)) \big)\Big) \\
	&\rightarrow_N \Big(\big((z\; z) \; y ) \; \big(y \; (z\; z)  \big)\Big)
\end{aligned}
$$

Note that normal order reduction required evaluation of the $\lambda w$ abstraction _twice_ before getting the final answer. Applicative order ($A$) reduction on the other hand will first reduce the $\lambda w$ expression to $(z\; z)$ before substituting it into $x$:

$$
\begin{aligned}
	\lambda x.\big((x \; y ) \; (y \; x )\big) \; \big(\lambda w.(w \; w )\; z)) &\rightarrow_A \lambda x.\big((x \; y ) \; (y \; x )\big) \; (z\; z) \\
	&\rightarrow_A \big((z\; z) \; y \big) \; \big(y \; (z\; z) \big) \\
\end{aligned}
$$

While applicative order seems like the clear winner in this example, it's trivial find a counter example (any recursive function) for which applicative order reduction will not only lose to normal order reduction, but may fail reduce to a β-normal form entirely, e.g.

$$
\begin{aligned}
\lambda x.m \; \big(\lambda x. (x\;x) \; \lambda x. (x\;x)\big)
	&\rightarrow_N \lambda x.m \; \Big(\big(\lambda x. (x\;x)\big) \;  \big(\lambda x. (x\;x)\big)\Big) \\
	&\rightarrow_N m
\end{aligned}
$$

whereas applicative order will attempt to unroll the fork bomb before realizing that the outermost $\lambda x$ discards its argument entirely. 

### Other Reduction Strategies

Along with normal and applicative order, other common strategies include:

- **Head Spine** (HSP): leftmost outermost, abstractions only reduced when in the head position,
- **Hybrid Normal** (HNO): a mix between Head Spine and Normal,
- **Call by Value** (CBV): leftmost innermost, no reductions inside abstractions,
- **Hybrid Applicative** (HAP): a mix between Call by Value and Applicative order
- **Call by Name** (CBN): leftmost outermost, no reductions inside abstractions

The 7 reduction strategies produce reductions which fall into one of four normal forms: NF (TL), Weak NF (TR), Head NF (BL), WHNF (BR)

TODO: cite https://www.cs.cornell.edu/courses/cs6110/2014sp/Handouts/Sestoft.pdf

<table>
  <tr>
	<td rowspan="2">reduce args</td>
    <td colspan="2">reduce under abstraction</td>
  </tr>
  <tr>
    <td>yes</td>
    <td>no</td>
  </tr>  
  <tr>
    <td>yes</td>
    <td>APP, NOR, HAP, HNO</td>
    <td>CBV</td>
  </tr>
<tr>
    <td>no</td>
	<td>HSP</td>
	<td>CBN</td>
  </tr>
</table>

We can further categorize reduction strategies as **pure** or **uniform** (those which only involve that reduction strategy itself) vs. hybrid (those which use a uniform strategy for the reduction of an abstraction $x$ in an application $(x \; y)$). 

| hybrid | uniform |
| ------ | ------- |
| NOR    | CBN     |
| HAP    | CBV     |
| HNO    | HSP     |


> So my sieve works for a finite amount of primes
> I found the issue
 It’s the reduction strategy
 (because there’s several)
 And because of the nested recursive calls (to `union` and `merge` and `primes` itself), I would have to define a custom reduction strategy
 e.g. doing it by hand it works
however, to get even just 2,3,5,7 it takes 1.1mil reductions

  
> [will]
> There isn't a "Don't be stupid" strategy built-in?


> Not really… like normal order reduction is the laziest, which is usually safe
 except when you reduce to a point where a recursive call is in the “head” position, but it would be more beneficial to unroll the argument to the head instead of the head
> which I have at least one instance of
> `multiples`  and `primes` both spawn infinite lists on the same “line”


excerpt from the paper

> and while I get why this highlighted part is crucial and should lend itself to normal order reduction, I think you really want to unroll “one step” of multiples and primes in lockstep at a time
 BFS the redux tree so to speak
but then at the same time, you need to prioritize a `take n` which will be in the head position, and so the trick is to get like 5 “steps” into `multiples` and `primes` each (exactly like we had on the whiteboard) to give you enough of a stream to `take` from
and there’s no general reduction order for this hyper specific case lol
