; Finite-list definitions instantiated independently for each element sort.
({declare_datatype} {list_sort}
  (({nil})
   ({cons} ({head} {element_sort}) ({tail} {list_sort}))))

({define_recursive} {app} (({xs} {list_sort}) ({ys} {list_sort})) {list_sort}
  ({ite} ((_ is {nil}) {xs})
       {ys}
       ({cons} ({head} {xs}) ({app} ({tail} {xs}) {ys}))))

({define_recursive} {rev} (({xs} {list_sort})) {list_sort}
  ({ite} ((_ is {nil}) {xs})
       {nil}
       ({app} ({rev} ({tail} {xs}))
                    ({cons} ({head} {xs}) {nil}))))

({define_recursive} {len} (({xs} {list_sort})) {integer_sort}
  ({ite} ((_ is {nil}) {xs})
       0
       ({add} 1 ({len} ({tail} {xs})))))

({define_recursive} {mem} (({xs} {list_sort}) ({x} {element_sort})) {boolean_sort}
  ({ite} ((_ is {nil}) {xs})
       {false_value}
       ({or} ({equal} ({head} {xs}) {x}) ({mem} ({tail} {xs}) {x}))))
