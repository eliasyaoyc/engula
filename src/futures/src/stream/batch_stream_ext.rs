// Copyright 2021 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use super::BatchStream;

pub trait BatchStreamExt: BatchStream {
    fn next_batch(&mut self, n: usize) -> NextBatchFuture<'_, Self>
    where
        Self: Unpin,
        Self::Batch: Unpin,
    {
        NextBatchFuture::new(self, n)
    }
}

impl<T: BatchStream + ?Sized> BatchStreamExt for T {}

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct NextBatchFuture<'a, T: ?Sized> {
    inner: &'a mut T,
    n: usize,
}

impl<T: ?Sized + Unpin> Unpin for NextBatchFuture<'_, T> {}

impl<'a, T> NextBatchFuture<'a, T>
where
    T: BatchStream + ?Sized + Unpin,
{
    fn new(inner: &'a mut T, n: usize) -> Self {
        Self { inner, n }
    }
}

impl<T> Future for NextBatchFuture<'_, T>
where
    T: BatchStream + ?Sized + Unpin,
{
    type Output = T::Batch;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        Pin::new(&mut this.inner).poll_next_batch(cx, this.n)
    }
}