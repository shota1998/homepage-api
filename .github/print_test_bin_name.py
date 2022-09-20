import os, sys, json; 
test_bin_path = json.load(sys.stdin)['executable']; 
test_bin_name = (os.path.basename(test_bin_path));
print (test_bin_name);
