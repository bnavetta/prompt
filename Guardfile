guard :shell do
	watch(/^src\/(.*)\.rs/) do |m|
		result = `cargo test`
		if result
			n m[0], "Build Succeeded", :success
		else
			n m[0], "Build Failed", :failed
		end
	end
end
