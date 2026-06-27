; Finite-list definitions instantiated independently for each element sort.
({declare_datatype} {list_sort}
  (({nil})
   ({cons} ({head} {element_sort}) ({tail} {list_sort}))))

({define_recursive} {app} (({xs} {list_sort}) ({ys} {list_sort})) {list_sort}
  ({ite} ((_ is {nil}) {xs})
       {ys}
       ({cons} ({head} {xs}) ({app} ({tail} {xs}) {ys}))))
