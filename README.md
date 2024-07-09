### 项目构建
```shell
cargo build --release
```

### 如何使用
step-1: 是否启动新版本计算逻辑[y/n]:
在 hive 1.2.1 前后的计算逻辑是不一致的，同时在 spark on hive 模式中 sparksql 所使用的是 1.2.1 之后的逻辑
因此 y 代表应用新逻辑，n 代表应用老逻辑

step-2: 输入 group by 后的列表[以;结尾]:
参考的输入案例: 
```
employee_code, employee_name, employer_code, employer_name, region_code, area_name, war_zone_name, datatype, inc_day;
```

step-3: 输入 grouping sets (*)的列表[以;结尾，按回车分割]:
参考的输入案例:
```
(employee_code, employee_name, employer_code, employer_name, region_code, area_name, war_zone_name, datatype, inc_day),
(employee_code, employee_name, employer_code, employer_name, region_code, area_name, war_zone_name, inc_day),
(employer_code, employer_name, region_code, area_name, war_zone_name, datatype, inc_day),
(employer_code, employer_name, region_code, area_name, war_zone_name , inc_day),
(region_code, area_name, war_zone_name, datatype, inc_day),
(region_code, area_name, war_zone_name, inc_day),
(war_zone_name, datatype, inc_day),
(war_zone_name, inc_day),
(datatype, inc_day),
(inc_day);
```
最终结果如下
![result](img/result.png)