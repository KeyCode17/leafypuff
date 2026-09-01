package com.leafypuff.ui.editor

import com.leafypuff.ui.photo.EntryPhoto

data class OpenedEntry(val draft: EntryDraft, val photos: List<EntryPhoto>)
