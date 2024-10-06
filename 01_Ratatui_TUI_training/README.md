- https://github.com/ratatui/ratatui-website/tree/main/code/tutorials/counter-app-basic

<hr />

# Writer
- Write a buffer into this writer, returning how many bytes were written.

- This function will attempt to write the entire contents of `buf`, but the entire write might not succeed, or the write may also generate an error. Typically, a call to `write` represents one attempt to write to any wrapped object.

- Calls to `write` are not guaranteed to block waiting for data to be written, and a write which would otherwise block can be indicated through an [`Err`] variant.

- If this method consumed `n > 0` bytes of `buf` it must return [`Ok(n)`]. If the return value is `Ok(n)` then `n` must satisfy `n <= buf.len()`. A return value of `Ok(0)` typically means that the underlying object is no longer able to accept bytes and will likely not be able to in the future as well, or that the buffer provided is empty.
 
# Errors
 
- Each call to `write` may generate an I/O error indicating that the operation could not be completed. If an error is returned then no bytes in the buffer were written to this writer.
- It is **not** considered an error if the entire buffer could not be written to this writer.
- An error of the [`ErrorKind::Interrupted`] kind is non-fatal and the write operation should be retried if there is nothing else to do.

이 작성기에 버퍼를 작성하여 작성한 바이트 수를 반환합니다.

 이 함수는 'buff'의 전체 내용을 작성하려고 시도하지만
 전체 쓰기가 성공하지 못하거나 쓰기에 오류가 발생할 수도 있습니다. 일반적으로 '쓰기' 호출은 랩핑된 개체에 쓰기를 시도하는 한 번의 시도를 나타냅니다.

  '쓰기' 요청은 데이터 쓰기 대기를 차단하는 것을 보장하지 않으며, 그렇지 않으면 차단할 수 있는 쓰기를 다음을 통해 표시할 수 있습니다
 ['에러'] 변형.

  이 방법이 'n > 0'바이트의 '버프'를 소비했다면 ['OK(n)']를 반환해야 합니다.
반환 값이 'Ok(n)'이면 'n'은 'n <= buf.len ()'를 만족해야 합니다.
반환 값이 'Ok(0)'이면 일반적으로 기본 개체가 더 이상 바이트를 수락할 수 없으며 앞으로도 수락할 수 없을 가능성이 높거나 제공된 버퍼가 비어 있음을 의미합니다.
 
# 오류
 
  '쓰기'를 호출할 때마다 작업을 완료할 수 없음을 나타내는 I/O 오류가 발생할 수 있습니다. 오류가 반환되면 버퍼의 바이트가 이 작성자에게 기록되지 않습니다.

 전체 버퍼를 이 작성자에게 쓸 수 없는 경우 **not**는 오류로 간주되지 않습니다.

 ['ErrorKind::interrupted'] 종류의 오류는 치명적이지 않으며 다른 작업이 없는 경우 쓰기 작업을 다시 시도해야 합니다.

<hr />
 

