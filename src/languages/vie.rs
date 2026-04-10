use super::Language;

/// Vietnamese common words for language detection.
pub fn language() -> Language {
	Language {
		english_name: "Vietnamese",
		iso_639_1: "vi",
		iso_639_2: "vie",
		bcp47: None,
		common_words: &[
			"và", "của", "có", "không", "là", "được", "trong", "một", "cho", "với", "những", "các", "đã",
			"tôi", "anh", "chị", "em", "nó", "chúng", "ta", "bạn", "họ", "ông", "bà", "cô", "chú", "bác",
			"này", "đó", "kia", "đâu", "gì", "ai", "khi", "nào", "sao", "thế", "nào", "vì", "bởi", "tại",
			"nên", "để", "thì", "mà", "nhưng", "tuy", "hoặc", "hay", "cũng", "lại", "vẫn", "còn", "mới",
			"đang", "sẽ", "đi", "đến", "về", "lên", "xuống", "ra", "vào", "ở", "làm", "nói", "hỏi",
			"trả", "lời", "biết", "hiểu", "thấy", "nhìn", "nghe", "nghĩ", "muốn", "cần", "phải", "nên",
			"có", "thể", "bắt", "đầu", "kết", "thúc", "xong", "hết", "rồi", "nữa", "thêm", "bớt",
			"nhiều", "ít", "một", "ít", "vài", "mọi", "tất", "cả", "mỗi", "từng", "cùng", "nhau",
			"riêng", "chung", "toàn", "bộ", "phần", "lớn", "hầu", "hết", "đa", "số", "thiểu", "số",
			"một", "số", "đôi", "chút",
		],
	}
}
