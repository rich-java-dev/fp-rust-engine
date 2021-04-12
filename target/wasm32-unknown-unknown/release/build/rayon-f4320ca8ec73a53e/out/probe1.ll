; ModuleID = 'probe1.3a1fbbbh-cgu.0'
source_filename = "probe1.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
target triple = "wasm32-unknown-unknown"

%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>" = type { [0 x i32], %"std::iter::StepBy<std::ops::Range<i32>>", [0 x i32] }
%"std::iter::StepBy<std::ops::Range<i32>>" = type { [0 x i32], { i32, i32 }, [0 x i32], i32, [0 x i8], i8, [3 x i8] }
%"std::panic::Location" = type { [0 x i32], { [0 x i8]*, i32 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }

@alloc1 = private unnamed_addr constant <{ [27 x i8] }> <{ [27 x i8] c"assertion failed: step != 0" }>, align 1
@alloc2 = private unnamed_addr constant <{ [125 x i8] }> <{ [125 x i8] c"/home/rich/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/step_by.rs" }>, align 1
@alloc3 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [125 x i8] }>, <{ [125 x i8] }>* @alloc2, i32 0, i32 0, i32 0), [12 x i8] c"}\00\00\00\15\00\00\00\09\00\00\00" }>, align 4

; core::iter::traits::iterator::Iterator::rev
; Function Attrs: inlinehint nounwind
define hidden void @_ZN4core4iter6traits8iterator8Iterator3rev17hb507c24ef79638e3E(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>"* noalias nocapture sret(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>") dereferenceable(16) %0, %"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture dereferenceable(16) %self) unnamed_addr #0 {
start:
  %_2 = alloca %"std::iter::StepBy<std::ops::Range<i32>>", align 4
  %1 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %self to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %1, i8* align 4 %2, i32 16, i1 false)
; call core::iter::adapters::rev::Rev<T>::new
  call void @"_ZN4core4iter8adapters3rev12Rev$LT$T$GT$3new17h0f70da8df2730e56E"(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>"* noalias nocapture sret(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>") dereferenceable(16) %0, %"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture dereferenceable(16) %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::iter::traits::iterator::Iterator::step_by
; Function Attrs: inlinehint nounwind
define hidden void @_ZN4core4iter6traits8iterator8Iterator7step_by17h44c0b1dff152feedE(%"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture sret(%"std::iter::StepBy<std::ops::Range<i32>>") dereferenceable(16) %0, i32 %self.0, i32 %self.1, i32 %step) unnamed_addr #0 {
start:
; call core::iter::adapters::step_by::StepBy<I>::new
  call void @"_ZN4core4iter8adapters7step_by15StepBy$LT$I$GT$3new17h46da446566d772e1E"(%"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture sret(%"std::iter::StepBy<std::ops::Range<i32>>") dereferenceable(16) %0, i32 %self.0, i32 %self.1, i32 %step)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::iter::adapters::rev::Rev<T>::new
; Function Attrs: nounwind
define hidden void @"_ZN4core4iter8adapters3rev12Rev$LT$T$GT$3new17h0f70da8df2730e56E"(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>"* noalias nocapture sret(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>") dereferenceable(16) %0, %"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture dereferenceable(16) %iter) unnamed_addr #1 {
start:
  %_2 = alloca %"std::iter::StepBy<std::ops::Range<i32>>", align 4
  %1 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %iter to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %1, i8* align 4 %2, i32 16, i1 false)
  %3 = bitcast %"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>"* %0 to %"std::iter::StepBy<std::ops::Range<i32>>"*
  %4 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %3 to i8*
  %5 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %_2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %4, i8* align 4 %5, i32 16, i1 false)
  ret void
}

; core::iter::adapters::step_by::StepBy<I>::new
; Function Attrs: nounwind
define hidden void @"_ZN4core4iter8adapters7step_by15StepBy$LT$I$GT$3new17h46da446566d772e1E"(%"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture sret(%"std::iter::StepBy<std::ops::Range<i32>>") dereferenceable(16) %0, i32 %iter.0, i32 %iter.1, i32 %step) unnamed_addr #1 {
start:
  %_4 = icmp ne i32 %step, 0
  %_3 = xor i1 %_4, true
  br i1 %_3, label %bb1, label %bb2

bb1:                                              ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hc64f9753b8a1be76E([0 x i8]* nonnull align 1 bitcast (<{ [27 x i8] }>* @alloc1 to [0 x i8]*), i32 27, %"std::panic::Location"* align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc3 to %"std::panic::Location"*))
  unreachable

bb2:                                              ; preds = %start
  %_7 = sub i32 %step, 1
  %1 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %0 to { i32, i32 }*
  %2 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %1, i32 0, i32 0
  store i32 %iter.0, i32* %2, align 4
  %3 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %1, i32 0, i32 1
  store i32 %iter.1, i32* %3, align 4
  %4 = getelementptr inbounds %"std::iter::StepBy<std::ops::Range<i32>>", %"std::iter::StepBy<std::ops::Range<i32>>"* %0, i32 0, i32 3
  store i32 %_7, i32* %4, align 4
  %5 = getelementptr inbounds %"std::iter::StepBy<std::ops::Range<i32>>", %"std::iter::StepBy<std::ops::Range<i32>>"* %0, i32 0, i32 5
  store i8 1, i8* %5, align 4
  ret void
}

; probe1::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe15probe17h2bb8a765c3877196E() unnamed_addr #1 {
start:
  %_3 = alloca { i32, i32 }, align 4
  %_2 = alloca %"std::iter::StepBy<std::ops::Range<i32>>", align 4
  %_1 = alloca %"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>", align 4
  %0 = bitcast { i32, i32 }* %_3 to i32*
  store i32 0, i32* %0, align 4
  %1 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  store i32 10, i32* %1, align 4
  %2 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 0
  %3 = load i32, i32* %2, align 4
  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  %5 = load i32, i32* %4, align 4
; call core::iter::traits::iterator::Iterator::step_by
  call void @_ZN4core4iter6traits8iterator8Iterator7step_by17h44c0b1dff152feedE(%"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture sret(%"std::iter::StepBy<std::ops::Range<i32>>") dereferenceable(16) %_2, i32 %3, i32 %5, i32 2)
  br label %bb1

bb1:                                              ; preds = %start
; call core::iter::traits::iterator::Iterator::rev
  call void @_ZN4core4iter6traits8iterator8Iterator3rev17hb507c24ef79638e3E(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>"* noalias nocapture sret(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>") dereferenceable(16) %_1, %"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture dereferenceable(16) %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; Function Attrs: argmemonly nofree nosync nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #2

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare void @_ZN4core9panicking5panic17hc64f9753b8a1be76E([0 x i8]* nonnull align 1, i32, %"std::panic::Location"* align 4 dereferenceable(16)) unnamed_addr #3

attributes #0 = { inlinehint nounwind "target-cpu"="generic" }
attributes #1 = { nounwind "target-cpu"="generic" }
attributes #2 = { argmemonly nofree nosync nounwind willreturn }
attributes #3 = { cold noinline noreturn nounwind "target-cpu"="generic" }
