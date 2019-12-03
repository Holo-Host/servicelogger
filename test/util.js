
// Safely gets a path `p == ['a', 0, 'b', ...] `on an object `o`, returning the
// indexed Object, or `null` if not available
const get = (p, o) =>  p.reduce((xs, x) => (xs && xs[x]) ? xs[x] : null, o)

module.exports = {
  get
}
