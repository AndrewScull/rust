# x86_64-linux-dios configuration
CC_x86_64-linux-dios=$(CC)
CXX_x86_64-linux-dios=$(CXX)
CPP_x86_64-linux-dios=$(CPP)
AR_x86_64-linux-dios=$(AR)
CFG_LIB_NAME_x86_64-linux-dios=lib$(1).so
CFG_STATIC_LIB_NAME_x86_64-linux-dios=lib$(1).a
CFG_LIB_GLOB_x86_64-linux-dios=lib$(1)-*.so
CFG_LIB_DSYM_GLOB_x86_64-linux-dios=lib$(1)-*.dylib.dSYM
CFG_JEMALLOC_CFLAGS_x86_64-linux-dios := -m64
CFG_GCCISH_CFLAGS_x86_64-linux-dios := -Wall -Werror -g -fPIC -m64
CFG_GCCISH_CXXFLAGS_x86_64-linux-dios := -fno-rtti
CFG_GCCISH_LINK_FLAGS_x86_64-linux-dios := -static -fPIC -g -m64
CFG_GCCISH_DEF_FLAG_x86_64-linux-dios := -Wl,--export-dynamic,--dynamic-list=
CFG_GCCISH_PRE_LIB_FLAGS_x86_64-linux-dios := -Wl,-whole-archive
CFG_GCCISH_POST_LIB_FLAGS_x86_64-linux-dios := -Wl,-no-whole-archive
CFG_DEF_SUFFIX_x86_64-linux-dios := .dios.def
CFG_LLC_FLAGS_x86_64-linux-dios :=
CFG_INSTALL_NAME_x86_64-linux-dios =
CFG_EXE_SUFFIX_x86_64-linux-dios =
CFG_WINDOWSY_x86_64-linux-dios :=
CFG_UNIXY_x86_64-linux-dios := 1
CFG_PATH_MUNGE_x86_64-linux-dios := true
CFG_LDPATH_x86_64-linux-dios :=
CFG_RUN_x86_64-linux-dios=$(2)
CFG_RUN_TARG_x86_64-linux-dios=$(call CFG_RUN_x86_64-linux-dios,,$(2))
CFG_GNU_TRIPLE_x86_64-linux-dios := x86_64-linux-dios

