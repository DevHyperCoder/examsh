<script lang="ts">
	import type { QuestionType } from './types/Exam';

	export let qtype: QuestionType;

	const badgeNames: Record<QuestionType, string> = {
		MultipleChoiceQuestion: 'MCQ',
		RawQuestion: 'RAW',
		PredictOutputQuestion: 'PREDICT'
	};
	const badgeColours: Record<QuestionType, string> = {
		MultipleChoiceQuestion: 'bg-violet-500',
		RawQuestion: 'bg-amber-500',
		PredictOutputQuestion: 'bg-green-500'
	};
</script>

<span class={`font-bold text-white px-2 py-1 rounded-md ${badgeColours[qtype]}`}
	>{badgeNames[qtype]}</span
>
