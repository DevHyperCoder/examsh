export interface ExamSchema {
	course_name: string;
	test_name: string;
	instructions: string;
	marking_instructions: string;
	date_fmt: string | null;
}
