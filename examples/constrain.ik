min=y>lim-|y?lim;
max=y<lim-|y?lim;
constrain=min(lim=upper, y=max(lim=lower));
constrain(y=0, lower=5, upper=15)+":"+constrain(y=10, lower=5, upper=15)+":"+constrain(y=20, lower=5, upper=15)
