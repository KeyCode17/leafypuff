package com.leafypuff.ui.editor

import com.leafypuff.ui.photo.EntryPhoto

/**
 * What reopening an entry hands the editor. The photos come back as decoded thumbnails rather than
 * ids because the editor draws them, and it has no way to reach the photo store itself.
 */
data class OpenedEntry(val draft: EntryDraft, val photos: List<EntryPhoto>)
