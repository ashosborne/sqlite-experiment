//! GENERATED run-16 file-batch replay: each durable case reproduced through
//! the Rust file engine and asserted equal to the frozen C golden.
use sqlite3_rust_spine::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::ptr;

fn golden(feat:&str,cnum:&str)->String{let mut p=PathBuf::from(env!("CARGO_MANIFEST_DIR"));p.pop();p.push(format!("tests/characterization/engine-files/cases/{feat}/{cnum}.approved.txt"));std::fs::read_to_string(&p).unwrap()}
struct Cap<'a>{cid:&'a str,rows:i32,lines:&'a mut Vec<String>}
unsafe extern "C" fn cb(a:*mut c_void,ac:c_int,av:*mut *mut c_char,_z:*mut *mut c_char)->c_int{let c=&mut *(a as *mut Cap);for i in 0..ac as usize{let p=*av.add(i);let v=if p.is_null(){"NULL".to_string()}else{CStr::from_ptr(p).to_str().unwrap().to_string()};c.lines.push(format!("OBS {} row{}.col{} {}",c.cid,c.rows,i,v));}c.rows+=1;0}
unsafe fn wr(l:&mut Vec<String>,cid:&str,label:&str,path:&str,sql:&str){let mut db:*mut Sqlite3=ptr::null_mut();let p=CString::new(path).unwrap();let mut rc=sqlite3_open(p.as_ptr(),&mut db);if rc==0{let c=CString::new(sql).unwrap();rc=sqlite3_exec(db,c.as_ptr(),None,ptr::null_mut(),ptr::null_mut());}sqlite3_close(db);l.push(format!("OBS {cid} {label} {rc}"));}
unsafe fn rd(l:&mut Vec<String>,cid:&str,path:&str,sql:&str){let mut db:*mut Sqlite3=ptr::null_mut();let p=CString::new(path).unwrap();let o=sqlite3_open(p.as_ptr(),&mut db);l.push(format!("OBS {cid} reopen.rc {o}"));let mut cap=Cap{cid,rows:0,lines:l};let c=CString::new(sql).unwrap();let rc=sqlite3_exec(db,c.as_ptr(),Some(cb),&mut cap as *mut Cap as *mut c_void,ptr::null_mut());let rows=cap.rows;l.push(format!("OBS {cid} read.rc {rc}"));l.push(format!("OBS {cid} cb.rows {rows}"));sqlite3_close(db);}

#[test]
fn engine_files_002_C001() {
  let path=format!("/tmp/eftest/fb_0_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-002-C001","write.rc",&path,"CREATE TABLE big(a INTEGER); INSERT INTO big VALUES (3),(6),(9),(12),(15),(18),(21),(24),(27),(30),(33),(36),(39),(42),(45),(48),(51),(54),(57),(60),(63),(66),(69),(72),(75),(78),(81),(84),(87),(90),(93),(96),(99),(102),(105),(108),(111),(114),(117),(120),(123),(126),(129),(132),(135),(138),(141),(144),(147),(150),(153),(156),(159),(162),(165),(168),(171),(174),(177),(180),(183),(186),(189),(192),(195),(198),(201),(204),(207),(210),(213),(216),(219),(222),(225),(228),(231),(234),(237),(240),(243),(246),(249),(252),(255),(258),(261),(264),(267),(270),(273),(276),(279),(282),(285),(288),(291),(294),(297),(300),(303),(306),(309),(312),(315),(318),(321),(324),(327),(330),(333),(336),(339),(342),(345),(348),(351),(354),(357),(360),(363),(366),(369),(372),(375),(378),(381),(384),(387),(390),(393),(396),(399),(402),(405),(408),(411),(414),(417),(420),(423),(426),(429),(432),(435),(438),(441),(444),(447),(450),(453),(456),(459),(462),(465),(468),(471),(474),(477),(480),(483),(486),(489),(492),(495),(498),(501),(504),(507),(510),(513),(516),(519),(522),(525),(528),(531),(534),(537),(540),(543),(546),(549),(552),(555),(558),(561),(564),(567),(570),(573),(576),(579),(582),(585),(588),(591),(594),(597),(600),(603),(606),(609),(612),(615),(618),(621),(624),(627),(630),(633),(636),(639),(642),(645),(648),(651),(654),(657),(660),(663),(666),(669),(672),(675),(678),(681),(684),(687),(690),(693),(696),(699),(702),(705),(708),(711),(714),(717),(720),(723),(726),(729),(732),(735),(738),(741),(744),(747),(750),(753),(756),(759),(762),(765),(768),(771),(774),(777),(780),(783),(786),(789),(792),(795),(798),(801),(804),(807),(810),(813),(816),(819),(822),(825),(828),(831),(834),(837),(840),(843),(846),(849),(852),(855),(858),(861),(864),(867),(870),(873),(876),(879),(882),(885),(888),(891),(894),(897),(900),(903),(906),(909),(912),(915),(918),(921),(924),(927),(930),(933),(936),(939),(942),(945),(948),(951),(954),(957),(960),(963),(966),(969),(972),(975),(978),(981),(984),(987),(990),(993),(996),(999),(1002),(1005),(1008),(1011),(1014),(1017),(1020),(1023),(1026),(1029),(1032),(1035),(1038),(1041),(1044),(1047),(1050),(1053),(1056),(1059),(1062),(1065),(1068),(1071),(1074),(1077),(1080),(1083),(1086),(1089),(1092),(1095),(1098),(1101),(1104),(1107),(1110),(1113),(1116),(1119),(1122),(1125),(1128),(1131),(1134),(1137),(1140),(1143),(1146),(1149),(1152),(1155),(1158),(1161),(1164),(1167),(1170),(1173),(1176),(1179),(1182),(1185),(1188),(1191),(1194),(1197),(1200),(1203),(1206),(1209),(1212),(1215),(1218),(1221),(1224),(1227),(1230),(1233),(1236),(1239),(1242),(1245),(1248),(1251),(1254),(1257),(1260),(1263),(1266),(1269),(1272),(1275),(1278),(1281),(1284),(1287),(1290),(1293),(1296),(1299),(1302),(1305),(1308),(1311),(1314),(1317),(1320),(1323),(1326),(1329),(1332),(1335),(1338),(1341),(1344),(1347),(1350),(1353),(1356),(1359),(1362),(1365),(1368),(1371),(1374),(1377),(1380),(1383),(1386),(1389),(1392),(1395),(1398),(1401),(1404),(1407),(1410),(1413),(1416),(1419),(1422),(1425),(1428),(1431),(1434),(1437),(1440),(1443),(1446),(1449),(1452),(1455),(1458),(1461),(1464),(1467),(1470),(1473),(1476),(1479),(1482),(1485),(1488),(1491),(1494),(1497),(1500),(1503),(1506),(1509),(1512),(1515),(1518),(1521),(1524),(1527),(1530),(1533),(1536),(1539),(1542),(1545),(1548),(1551),(1554),(1557),(1560),(1563),(1566),(1569),(1572),(1575),(1578),(1581),(1584),(1587),(1590),(1593),(1596),(1599),(1602),(1605),(1608),(1611),(1614),(1617),(1620),(1623),(1626),(1629),(1632),(1635),(1638),(1641),(1644),(1647),(1650),(1653),(1656),(1659),(1662),(1665),(1668),(1671),(1674),(1677),(1680),(1683),(1686),(1689),(1692),(1695),(1698),(1701),(1704),(1707),(1710),(1713),(1716),(1719),(1722),(1725),(1728),(1731),(1734),(1737),(1740),(1743),(1746),(1749),(1752),(1755),(1758),(1761),(1764),(1767),(1770),(1773),(1776),(1779),(1782),(1785),(1788),(1791),(1794),(1797),(1800),(1803),(1806),(1809),(1812),(1815),(1818),(1821),(1824),(1827),(1830),(1833),(1836),(1839),(1842),(1845),(1848),(1851),(1854),(1857),(1860),(1863),(1866),(1869),(1872),(1875),(1878),(1881),(1884),(1887),(1890),(1893),(1896),(1899),(1902),(1905),(1908),(1911),(1914),(1917),(1920),(1923),(1926),(1929),(1932),(1935),(1938),(1941),(1944),(1947),(1950),(1953),(1956),(1959),(1962),(1965),(1968),(1971),(1974),(1977),(1980),(1983),(1986),(1989),(1992),(1995),(1998),(2001),(2004),(2007),(2010),(2013),(2016),(2019),(2022),(2025),(2028),(2031),(2034),(2037),(2040),(2043),(2046),(2049),(2052),(2055),(2058),(2061),(2064),(2067),(2070),(2073),(2076),(2079),(2082),(2085),(2088),(2091),(2094),(2097),(2100),(2103),(2106),(2109),(2112),(2115),(2118),(2121),(2124),(2127),(2130),(2133),(2136),(2139),(2142),(2145),(2148),(2151),(2154),(2157),(2160),(2163),(2166),(2169),(2172),(2175),(2178),(2181),(2184),(2187),(2190),(2193),(2196),(2199),(2202),(2205),(2208),(2211),(2214),(2217),(2220),(2223),(2226),(2229),(2232),(2235),(2238),(2241),(2244),(2247),(2250),(2253),(2256),(2259),(2262),(2265),(2268),(2271),(2274),(2277),(2280),(2283),(2286),(2289),(2292),(2295),(2298),(2301),(2304),(2307),(2310),(2313),(2316),(2319),(2322),(2325),(2328),(2331),(2334),(2337),(2340),(2343),(2346),(2349),(2352),(2355),(2358),(2361),(2364),(2367),(2370),(2373),(2376),(2379),(2382),(2385),(2388),(2391),(2394),(2397),(2400),(2403),(2406),(2409),(2412),(2415),(2418),(2421),(2424),(2427),(2430),(2433),(2436),(2439),(2442),(2445),(2448),(2451),(2454),(2457),(2460),(2463),(2466),(2469),(2472),(2475),(2478),(2481),(2484),(2487),(2490),(2493),(2496),(2499),(2502),(2505),(2508),(2511),(2514),(2517),(2520),(2523),(2526),(2529),(2532),(2535),(2538),(2541),(2544),(2547),(2550),(2553),(2556),(2559),(2562),(2565),(2568),(2571),(2574),(2577),(2580),(2583),(2586),(2589),(2592),(2595),(2598),(2601),(2604),(2607),(2610),(2613),(2616),(2619),(2622),(2625),(2628),(2631),(2634),(2637),(2640),(2643),(2646),(2649),(2652),(2655),(2658),(2661),(2664),(2667),(2670),(2673),(2676),(2679),(2682),(2685),(2688),(2691),(2694),(2697),(2700),(2703),(2706),(2709),(2712),(2715),(2718),(2721),(2724),(2727),(2730),(2733),(2736),(2739),(2742),(2745),(2748),(2751),(2754),(2757),(2760),(2763),(2766),(2769),(2772),(2775),(2778),(2781),(2784),(2787),(2790),(2793),(2796),(2799),(2802),(2805),(2808),(2811),(2814),(2817),(2820),(2823),(2826),(2829),(2832),(2835),(2838),(2841),(2844),(2847),(2850),(2853),(2856),(2859),(2862),(2865),(2868),(2871),(2874),(2877),(2880),(2883),(2886),(2889),(2892),(2895),(2898),(2901),(2904),(2907),(2910),(2913),(2916),(2919),(2922),(2925),(2928),(2931),(2934),(2937),(2940),(2943),(2946),(2949),(2952),(2955),(2958),(2961),(2964),(2967),(2970),(2973),(2976),(2979),(2982),(2985),(2988),(2991),(2994),(2997),(3000),(3003),(3006),(3009),(3012),(3015),(3018),(3021),(3024),(3027),(3030),(3033),(3036),(3039),(3042),(3045),(3048),(3051),(3054),(3057),(3060),(3063),(3066),(3069),(3072),(3075),(3078),(3081),(3084),(3087),(3090),(3093),(3096),(3099),(3102),(3105),(3108),(3111),(3114),(3117),(3120),(3123),(3126),(3129),(3132),(3135),(3138),(3141),(3144),(3147),(3150),(3153),(3156),(3159),(3162),(3165),(3168),(3171),(3174),(3177),(3180),(3183),(3186),(3189),(3192),(3195),(3198),(3201),(3204),(3207),(3210),(3213),(3216),(3219),(3222),(3225),(3228),(3231),(3234),(3237),(3240),(3243),(3246),(3249),(3252),(3255),(3258),(3261),(3264),(3267),(3270),(3273),(3276),(3279),(3282),(3285),(3288),(3291),(3294),(3297),(3300),(3303),(3306),(3309),(3312),(3315),(3318),(3321),(3324),(3327),(3330),(3333),(3336),(3339),(3342),(3345),(3348),(3351),(3354),(3357),(3360),(3363),(3366),(3369),(3372),(3375),(3378),(3381),(3384),(3387),(3390),(3393),(3396),(3399),(3402),(3405),(3408),(3411),(3414),(3417),(3420),(3423),(3426),(3429),(3432),(3435),(3438),(3441),(3444),(3447),(3450),(3453),(3456),(3459),(3462),(3465),(3468),(3471),(3474),(3477),(3480),(3483),(3486),(3489),(3492),(3495),(3498),(3501),(3504),(3507),(3510),(3513),(3516),(3519),(3522),(3525),(3528),(3531),(3534),(3537),(3540),(3543),(3546),(3549),(3552),(3555),(3558),(3561),(3564),(3567),(3570),(3573),(3576),(3579),(3582),(3585),(3588),(3591),(3594),(3597),(3600);");
    rd(&mut l,"engine-files-002-C001",&path,"SELECT count(*) FROM big;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-002","C001"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_002_C002() {
  let path=format!("/tmp/eftest/fb_1_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-002-C002","write.rc",&path,"CREATE TABLE big(a INTEGER); INSERT INTO big VALUES (3),(6),(9),(12),(15),(18),(21),(24),(27),(30),(33),(36),(39),(42),(45),(48),(51),(54),(57),(60),(63),(66),(69),(72),(75),(78),(81),(84),(87),(90),(93),(96),(99),(102),(105),(108),(111),(114),(117),(120),(123),(126),(129),(132),(135),(138),(141),(144),(147),(150),(153),(156),(159),(162),(165),(168),(171),(174),(177),(180),(183),(186),(189),(192),(195),(198),(201),(204),(207),(210),(213),(216),(219),(222),(225),(228),(231),(234),(237),(240),(243),(246),(249),(252),(255),(258),(261),(264),(267),(270),(273),(276),(279),(282),(285),(288),(291),(294),(297),(300),(303),(306),(309),(312),(315),(318),(321),(324),(327),(330),(333),(336),(339),(342),(345),(348),(351),(354),(357),(360),(363),(366),(369),(372),(375),(378),(381),(384),(387),(390),(393),(396),(399),(402),(405),(408),(411),(414),(417),(420),(423),(426),(429),(432),(435),(438),(441),(444),(447),(450),(453),(456),(459),(462),(465),(468),(471),(474),(477),(480),(483),(486),(489),(492),(495),(498),(501),(504),(507),(510),(513),(516),(519),(522),(525),(528),(531),(534),(537),(540),(543),(546),(549),(552),(555),(558),(561),(564),(567),(570),(573),(576),(579),(582),(585),(588),(591),(594),(597),(600),(603),(606),(609),(612),(615),(618),(621),(624),(627),(630),(633),(636),(639),(642),(645),(648),(651),(654),(657),(660),(663),(666),(669),(672),(675),(678),(681),(684),(687),(690),(693),(696),(699),(702),(705),(708),(711),(714),(717),(720),(723),(726),(729),(732),(735),(738),(741),(744),(747),(750),(753),(756),(759),(762),(765),(768),(771),(774),(777),(780),(783),(786),(789),(792),(795),(798),(801),(804),(807),(810),(813),(816),(819),(822),(825),(828),(831),(834),(837),(840),(843),(846),(849),(852),(855),(858),(861),(864),(867),(870),(873),(876),(879),(882),(885),(888),(891),(894),(897),(900),(903),(906),(909),(912),(915),(918),(921),(924),(927),(930),(933),(936),(939),(942),(945),(948),(951),(954),(957),(960),(963),(966),(969),(972),(975),(978),(981),(984),(987),(990),(993),(996),(999),(1002),(1005),(1008),(1011),(1014),(1017),(1020),(1023),(1026),(1029),(1032),(1035),(1038),(1041),(1044),(1047),(1050),(1053),(1056),(1059),(1062),(1065),(1068),(1071),(1074),(1077),(1080),(1083),(1086),(1089),(1092),(1095),(1098),(1101),(1104),(1107),(1110),(1113),(1116),(1119),(1122),(1125),(1128),(1131),(1134),(1137),(1140),(1143),(1146),(1149),(1152),(1155),(1158),(1161),(1164),(1167),(1170),(1173),(1176),(1179),(1182),(1185),(1188),(1191),(1194),(1197),(1200),(1203),(1206),(1209),(1212),(1215),(1218),(1221),(1224),(1227),(1230),(1233),(1236),(1239),(1242),(1245),(1248),(1251),(1254),(1257),(1260),(1263),(1266),(1269),(1272),(1275),(1278),(1281),(1284),(1287),(1290),(1293),(1296),(1299),(1302),(1305),(1308),(1311),(1314),(1317),(1320),(1323),(1326),(1329),(1332),(1335),(1338),(1341),(1344),(1347),(1350),(1353),(1356),(1359),(1362),(1365),(1368),(1371),(1374),(1377),(1380),(1383),(1386),(1389),(1392),(1395),(1398),(1401),(1404),(1407),(1410),(1413),(1416),(1419),(1422),(1425),(1428),(1431),(1434),(1437),(1440),(1443),(1446),(1449),(1452),(1455),(1458),(1461),(1464),(1467),(1470),(1473),(1476),(1479),(1482),(1485),(1488),(1491),(1494),(1497),(1500),(1503),(1506),(1509),(1512),(1515),(1518),(1521),(1524),(1527),(1530),(1533),(1536),(1539),(1542),(1545),(1548),(1551),(1554),(1557),(1560),(1563),(1566),(1569),(1572),(1575),(1578),(1581),(1584),(1587),(1590),(1593),(1596),(1599),(1602),(1605),(1608),(1611),(1614),(1617),(1620),(1623),(1626),(1629),(1632),(1635),(1638),(1641),(1644),(1647),(1650),(1653),(1656),(1659),(1662),(1665),(1668),(1671),(1674),(1677),(1680),(1683),(1686),(1689),(1692),(1695),(1698),(1701),(1704),(1707),(1710),(1713),(1716),(1719),(1722),(1725),(1728),(1731),(1734),(1737),(1740),(1743),(1746),(1749),(1752),(1755),(1758),(1761),(1764),(1767),(1770),(1773),(1776),(1779),(1782),(1785),(1788),(1791),(1794),(1797),(1800),(1803),(1806),(1809),(1812),(1815),(1818),(1821),(1824),(1827),(1830),(1833),(1836),(1839),(1842),(1845),(1848),(1851),(1854),(1857),(1860),(1863),(1866),(1869),(1872),(1875),(1878),(1881),(1884),(1887),(1890),(1893),(1896),(1899),(1902),(1905),(1908),(1911),(1914),(1917),(1920),(1923),(1926),(1929),(1932),(1935),(1938),(1941),(1944),(1947),(1950),(1953),(1956),(1959),(1962),(1965),(1968),(1971),(1974),(1977),(1980),(1983),(1986),(1989),(1992),(1995),(1998),(2001),(2004),(2007),(2010),(2013),(2016),(2019),(2022),(2025),(2028),(2031),(2034),(2037),(2040),(2043),(2046),(2049),(2052),(2055),(2058),(2061),(2064),(2067),(2070),(2073),(2076),(2079),(2082),(2085),(2088),(2091),(2094),(2097),(2100),(2103),(2106),(2109),(2112),(2115),(2118),(2121),(2124),(2127),(2130),(2133),(2136),(2139),(2142),(2145),(2148),(2151),(2154),(2157),(2160),(2163),(2166),(2169),(2172),(2175),(2178),(2181),(2184),(2187),(2190),(2193),(2196),(2199),(2202),(2205),(2208),(2211),(2214),(2217),(2220),(2223),(2226),(2229),(2232),(2235),(2238),(2241),(2244),(2247),(2250),(2253),(2256),(2259),(2262),(2265),(2268),(2271),(2274),(2277),(2280),(2283),(2286),(2289),(2292),(2295),(2298),(2301),(2304),(2307),(2310),(2313),(2316),(2319),(2322),(2325),(2328),(2331),(2334),(2337),(2340),(2343),(2346),(2349),(2352),(2355),(2358),(2361),(2364),(2367),(2370),(2373),(2376),(2379),(2382),(2385),(2388),(2391),(2394),(2397),(2400),(2403),(2406),(2409),(2412),(2415),(2418),(2421),(2424),(2427),(2430),(2433),(2436),(2439),(2442),(2445),(2448),(2451),(2454),(2457),(2460),(2463),(2466),(2469),(2472),(2475),(2478),(2481),(2484),(2487),(2490),(2493),(2496),(2499),(2502),(2505),(2508),(2511),(2514),(2517),(2520),(2523),(2526),(2529),(2532),(2535),(2538),(2541),(2544),(2547),(2550),(2553),(2556),(2559),(2562),(2565),(2568),(2571),(2574),(2577),(2580),(2583),(2586),(2589),(2592),(2595),(2598),(2601),(2604),(2607),(2610),(2613),(2616),(2619),(2622),(2625),(2628),(2631),(2634),(2637),(2640),(2643),(2646),(2649),(2652),(2655),(2658),(2661),(2664),(2667),(2670),(2673),(2676),(2679),(2682),(2685),(2688),(2691),(2694),(2697),(2700),(2703),(2706),(2709),(2712),(2715),(2718),(2721),(2724),(2727),(2730),(2733),(2736),(2739),(2742),(2745),(2748),(2751),(2754),(2757),(2760),(2763),(2766),(2769),(2772),(2775),(2778),(2781),(2784),(2787),(2790),(2793),(2796),(2799),(2802),(2805),(2808),(2811),(2814),(2817),(2820),(2823),(2826),(2829),(2832),(2835),(2838),(2841),(2844),(2847),(2850),(2853),(2856),(2859),(2862),(2865),(2868),(2871),(2874),(2877),(2880),(2883),(2886),(2889),(2892),(2895),(2898),(2901),(2904),(2907),(2910),(2913),(2916),(2919),(2922),(2925),(2928),(2931),(2934),(2937),(2940),(2943),(2946),(2949),(2952),(2955),(2958),(2961),(2964),(2967),(2970),(2973),(2976),(2979),(2982),(2985),(2988),(2991),(2994),(2997),(3000),(3003),(3006),(3009),(3012),(3015),(3018),(3021),(3024),(3027),(3030),(3033),(3036),(3039),(3042),(3045),(3048),(3051),(3054),(3057),(3060),(3063),(3066),(3069),(3072),(3075),(3078),(3081),(3084),(3087),(3090),(3093),(3096),(3099),(3102),(3105),(3108),(3111),(3114),(3117),(3120),(3123),(3126),(3129),(3132),(3135),(3138),(3141),(3144),(3147),(3150),(3153),(3156),(3159),(3162),(3165),(3168),(3171),(3174),(3177),(3180),(3183),(3186),(3189),(3192),(3195),(3198),(3201),(3204),(3207),(3210),(3213),(3216),(3219),(3222),(3225),(3228),(3231),(3234),(3237),(3240),(3243),(3246),(3249),(3252),(3255),(3258),(3261),(3264),(3267),(3270),(3273),(3276),(3279),(3282),(3285),(3288),(3291),(3294),(3297),(3300),(3303),(3306),(3309),(3312),(3315),(3318),(3321),(3324),(3327),(3330),(3333),(3336),(3339),(3342),(3345),(3348),(3351),(3354),(3357),(3360),(3363),(3366),(3369),(3372),(3375),(3378),(3381),(3384),(3387),(3390),(3393),(3396),(3399),(3402),(3405),(3408),(3411),(3414),(3417),(3420),(3423),(3426),(3429),(3432),(3435),(3438),(3441),(3444),(3447),(3450),(3453),(3456),(3459),(3462),(3465),(3468),(3471),(3474),(3477),(3480),(3483),(3486),(3489),(3492),(3495),(3498),(3501),(3504),(3507),(3510),(3513),(3516),(3519),(3522),(3525),(3528),(3531),(3534),(3537),(3540),(3543),(3546),(3549),(3552),(3555),(3558),(3561),(3564),(3567),(3570),(3573),(3576),(3579),(3582),(3585),(3588),(3591),(3594),(3597),(3600);");
    rd(&mut l,"engine-files-002-C002",&path,"SELECT a FROM big WHERE a=1503;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-002","C002"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_002_C003() {
  let path=format!("/tmp/eftest/fb_2_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-002-C003","write.rc",&path,"CREATE TABLE wide(id INTEGER, t TEXT); INSERT INTO wide VALUES (1,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(2,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(3,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(4,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(5,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(6,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(7,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(8,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(9,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(10,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(11,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(12,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(13,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(14,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(15,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(16,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(17,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(18,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(19,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(20,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(21,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(22,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(23,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(24,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(25,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(26,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(27,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(28,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(29,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(30,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(31,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(32,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(33,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(34,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(35,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(36,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(37,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(38,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(39,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(40,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij');");
    rd(&mut l,"engine-files-002-C003",&path,"SELECT count(*) FROM wide;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-002","C003"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_002_C004() {
  let path=format!("/tmp/eftest/fb_3_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-002-C004","write.rc",&path,"CREATE TABLE wide(id INTEGER, t TEXT); INSERT INTO wide VALUES (1,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(2,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(3,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(4,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(5,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(6,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(7,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(8,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(9,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(10,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(11,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(12,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(13,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(14,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(15,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(16,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(17,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(18,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(19,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(20,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(21,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(22,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(23,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(24,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(25,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(26,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(27,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(28,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(29,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(30,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(31,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(32,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(33,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(34,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(35,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(36,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(37,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(38,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(39,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij'),(40,'abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij');");
    rd(&mut l,"engine-files-002-C004",&path,"SELECT t FROM wide WHERE id=20;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-002","C004"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_002_C005() {
  let path=format!("/tmp/eftest/fb_4_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-002-C005","write.rc",&path,"CREATE TABLE mix(a INTEGER, b TEXT); INSERT INTO mix VALUES (1,'v1'),(2,'v2'),(3,'v3'),(4,'v4'),(5,'v5'),(6,'v6'),(7,'v7'),(8,'v8'),(9,'v9'),(10,'v10'),(11,'v11'),(12,'v12'),(13,'v13'),(14,'v14'),(15,'v15'),(16,'v16'),(17,'v17'),(18,'v18'),(19,'v19'),(20,'v20'),(21,'v21'),(22,'v22'),(23,'v23'),(24,'v24'),(25,'v25'),(26,'v26'),(27,'v27'),(28,'v28'),(29,'v29'),(30,'v30'),(31,'v31'),(32,'v32'),(33,'v33'),(34,'v34'),(35,'v35'),(36,'v36'),(37,'v37'),(38,'v38'),(39,'v39'),(40,'v40');");
    rd(&mut l,"engine-files-002-C005",&path,"SELECT count(*) FROM mix;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-002","C005"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_002_C006() {
  let path=format!("/tmp/eftest/fb_5_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-002-C006","write.rc",&path,"CREATE TABLE s(a INTEGER, b TEXT); INSERT INTO s VALUES (1,'p'),(888002,'q'),(3,'r');");
    rd(&mut l,"engine-files-002-C006",&path,"SELECT a,b FROM s ORDER BY a;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-002","C006"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C001() {
  let path=format!("/tmp/eftest/fb_6_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C001","write.rc",&path,"PRAGMA foreign_keys=ON; CREATE TABLE par(id INTEGER PRIMARY KEY); CREATE TABLE chi(pid REFERENCES par(id)); INSERT INTO par VALUES(5); INSERT INTO chi VALUES(5);");
    wr(&mut l,"engine-files-003-C001","orphan.rc",&path,"PRAGMA foreign_keys=ON; INSERT INTO chi VALUES(6);");
    rd(&mut l,"engine-files-003-C001",&path,"SELECT count(*) FROM chi;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C001"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C002() {
  let path=format!("/tmp/eftest/fb_7_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C002","write.rc",&path,"PRAGMA foreign_keys=ON; CREATE TABLE par(id INTEGER PRIMARY KEY); CREATE TABLE chi(pid REFERENCES par(id)); INSERT INTO par VALUES(5),(6);");
    wr(&mut l,"engine-files-003-C002","valid.rc",&path,"PRAGMA foreign_keys=ON; INSERT INTO chi VALUES(6);");
    rd(&mut l,"engine-files-003-C002",&path,"SELECT pid FROM chi WHERE pid=6;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C002"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C003() {
  let path=format!("/tmp/eftest/fb_8_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C003","write.rc",&path,"PRAGMA foreign_keys=ON; CREATE TABLE p2(id INTEGER PRIMARY KEY); CREATE TABLE c2(pid REFERENCES p2(id) ON DELETE CASCADE); INSERT INTO p2 VALUES(1); INSERT INTO c2 VALUES(1);");
    wr(&mut l,"engine-files-003-C003","delete.rc",&path,"PRAGMA foreign_keys=ON; DELETE FROM p2 WHERE id=1;");
    rd(&mut l,"engine-files-003-C003",&path,"SELECT count(*) FROM c2;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C003"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C004() {
  let path=format!("/tmp/eftest/fb_9_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C004","write.rc",&path,"CREATE TABLE tr(a INTEGER); CREATE TABLE tlog(v INTEGER); CREATE TRIGGER trg AFTER INSERT ON tr BEGIN INSERT INTO tlog VALUES(new.a); END;");
    wr(&mut l,"engine-files-003-C004","fire.rc",&path,"INSERT INTO tr VALUES(7);");
    rd(&mut l,"engine-files-003-C004",&path,"SELECT v FROM tlog;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C004"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C005() {
  let path=format!("/tmp/eftest/fb_10_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C005","write.rc",&path,"CREATE TABLE tr(a INTEGER); CREATE TABLE tlog(v INTEGER); CREATE TRIGGER trg AFTER INSERT ON tr BEGIN INSERT INTO tlog VALUES(new.a); END;");
    rd(&mut l,"engine-files-003-C005",&path,"SELECT count(*) FROM sqlite_master WHERE type='trigger';");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C005"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C006() {
  let path=format!("/tmp/eftest/fb_11_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C006","write.rc",&path,"CREATE TABLE t3(a INTEGER); INSERT INTO t3 VALUES(9);");
    wr(&mut l,"engine-files-003-C006","alter.rc",&path,"ALTER TABLE t3 RENAME TO t3x;");
    rd(&mut l,"engine-files-003-C006",&path,"SELECT a FROM t3x;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C006"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C007() {
  let path=format!("/tmp/eftest/fb_12_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C007","write.rc",&path,"CREATE TABLE t4(a INTEGER); INSERT INTO t4 VALUES(9);");
    wr(&mut l,"engine-files-003-C007","alter.rc",&path,"ALTER TABLE t4 ADD COLUMN b DEFAULT 5;");
    rd(&mut l,"engine-files-003-C007",&path,"SELECT a,b FROM t4;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C007"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C008() {
  let path=format!("/tmp/eftest/fb_13_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C008","write.rc",&path,"CREATE TABLE up(a INTEGER PRIMARY KEY, b TEXT); INSERT INTO up VALUES(1,'x'); INSERT INTO up VALUES(1,'y') ON CONFLICT(a) DO NOTHING;");
    rd(&mut l,"engine-files-003-C008",&path,"SELECT b FROM up;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C008"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C009() {
  let path=format!("/tmp/eftest/fb_14_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C009","write.rc",&path,"CREATE TABLE up2(a INTEGER PRIMARY KEY, b TEXT); INSERT INTO up2 VALUES(1,'x'); INSERT INTO up2 VALUES(1,'y') ON CONFLICT(a) DO UPDATE SET b=excluded.b;");
    rd(&mut l,"engine-files-003-C009",&path,"SELECT b FROM up2;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C009"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C010() {
  let path=format!("/tmp/eftest/fb_15_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C010","write.rc",&path,"CREATE TABLE k(id INTEGER PRIMARY KEY, v TEXT); INSERT INTO k VALUES(42,'hi');");
    rd(&mut l,"engine-files-003-C010",&path,"SELECT id,v FROM k WHERE id=42;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C010"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C011() {
  let path=format!("/tmp/eftest/fb_16_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C011","write.rc",&path,"CREATE TABLE m3(a INTEGER, b TEXT, c INTEGER); INSERT INTO m3 VALUES(2,'y',20),(1,'x',10);");
    rd(&mut l,"engine-files-003-C011",&path,"SELECT a,b,c FROM m3 ORDER BY a;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C011"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_003_C012() {
  let path=format!("/tmp/eftest/fb_17_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-003-C012","write.rc",&path,"PRAGMA foreign_keys=ON; CREATE TABLE pp(id INTEGER PRIMARY KEY); CREATE TABLE cc(pid REFERENCES pp(id)); INSERT INTO pp VALUES(3); INSERT INTO cc VALUES(3);");
    rd(&mut l,"engine-files-003-C012",&path,"SELECT count(*) FROM pp;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-003","C012"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_004_C001() {
  let path=format!("/tmp/eftest/fb_18_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-004-C001","write.rc",&path,"CREATE TABLE ek(a INTEGER); INSERT INTO ek VALUES(7);");
    rd(&mut l,"engine-files-004-C001",&path,"SELECT a FROM ek;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-004","C001"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_004_C002() {
  let path=format!("/tmp/eftest/fb_19_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-004-C002","write.rc",&path,"CREATE TABLE ek2(a INTEGER, b TEXT); INSERT INTO ek2 VALUES(1,'x'); INSERT INTO ek2 VALUES(2,'y');");
    rd(&mut l,"engine-files-004-C002",&path,"SELECT a,b FROM ek2 ORDER BY a;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-004","C002"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_004_C003() {
  let path=format!("/tmp/eftest/fb_20_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-004-C003","write.rc",&path,"CREATE TABLE ek3(a INTEGER); INSERT INTO ek3 VALUES(10);");
    wr(&mut l,"engine-files-004-C003","upd.rc",&path,"UPDATE ek3 SET a=11;");
    rd(&mut l,"engine-files-004-C003",&path,"SELECT a FROM ek3;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-004","C003"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_004_C004() {
  let path=format!("/tmp/eftest/fb_21_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-004-C004","write.rc",&path,"CREATE TABLE ek4(a INTEGER); INSERT INTO ek4 VALUES(1),(2); DELETE FROM ek4 WHERE a=1;");
    rd(&mut l,"engine-files-004-C004",&path,"SELECT a FROM ek4;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-004","C004"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_004_C005() {
  let path=format!("/tmp/eftest/fb_22_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-004-C005","write.rc",&path,"CREATE TABLE ek5(a INTEGER); INSERT INTO ek5 VALUES(424243);");
    rd(&mut l,"engine-files-004-C005",&path,"SELECT a FROM ek5;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-004","C005"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_004_C006() {
  let path=format!("/tmp/eftest/fb_23_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-004-C006","write.rc",&path,"CREATE TABLE n1(a INTEGER); INSERT INTO n1 VALUES(5);");
    rd(&mut l,"engine-files-004-C006",&path,"SELECT n1.a, a, rowid FROM n1;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-004","C006"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_004_C007() {
  let path=format!("/tmp/eftest/fb_24_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-004-C007","write.rc",&path,"CREATE TABLE t2(a INTEGER); CREATE UNIQUE INDEX i2 ON t2(a); INSERT INTO t2 VALUES(1); INSERT OR IGNORE INTO t2 VALUES(1);");
    rd(&mut l,"engine-files-004-C007",&path,"SELECT count(*) FROM t2;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-004","C007"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_004_C008() {
  let path=format!("/tmp/eftest/fb_25_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-004-C008","write.rc",&path,"CREATE TABLE u(a INTEGER UNIQUE); INSERT INTO u VALUES(1); INSERT OR REPLACE INTO u VALUES(1); INSERT OR IGNORE INTO u VALUES(1);");
    rd(&mut l,"engine-files-004-C008",&path,"SELECT count(*) FROM u;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-004","C008"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_004_C009() {
  let path=format!("/tmp/eftest/fb_26_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-004-C009","write.rc",&path,"PRAGMA foreign_keys=ON; CREATE TABLE p2(id INTEGER PRIMARY KEY); CREATE TABLE c2(pid REFERENCES p2(id) ON DELETE CASCADE); INSERT INTO p2 VALUES(1); INSERT INTO c2 VALUES(1); DELETE FROM p2;");
    rd(&mut l,"engine-files-004-C009",&path,"SELECT count(*) FROM c2;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-004","C009"));
  let _=std::fs::remove_file(&path);
}

#[test]
fn engine_files_004_C010() {
  let path=format!("/tmp/eftest/fb_27_{}.db",std::process::id()); let _=std::fs::remove_file(&path);
  let mut l=Vec::new(); unsafe {
    wr(&mut l,"engine-files-004-C010","write.rc",&path,"CREATE TABLE tr2(a INTEGER); CREATE TABLE tlog2(v INTEGER); CREATE TRIGGER trg2 AFTER INSERT ON tr2 BEGIN INSERT INTO tlog2 VALUES(new.a*2); END; INSERT INTO tr2 VALUES(7);");
    rd(&mut l,"engine-files-004-C010",&path,"SELECT v FROM tlog2;");
  }
  assert_eq!(l.join("\n")+"\n", golden("engine-files-004","C010"));
  let _=std::fs::remove_file(&path);
}
