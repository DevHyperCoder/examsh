import type {ExamSchema} from "./ExamSchema";

export interface Exam {
    exam_schema: ExamSchema,
    questions: Question[]
}

export type Question = MultipleChoiceQuestion | RawQuestion | PredictOutputQuestion;
export type QuestionType = "PredictOutputQuestion" | "MultipleChoiceQuestion" |  "RawQuestion";

export interface MultipleChoiceQuestion {
    qtype: "MultipleChoiceQuestion",
    answers: string[],
    question: string,
    correct_id: number
}

export interface RawQuestion {
    qtype: "RawQuestion",
    question: string;
}

export interface PredictOutputQuestion {
    qtype: "PredictOutputQuestion";
    question: string;
    pre_run: string;
    run: string;
    post_run: string;
    _code: [filename: string, code: string][];
}
