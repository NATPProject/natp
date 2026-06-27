; Finite-list definitions instantiated independently for each element sort.
({declare_datatype} {list_sort}
  (({nil})
   ({cons} ({head} {element_sort}) ({tail} {list_sort}))))
