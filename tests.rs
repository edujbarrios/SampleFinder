
	// Test the search for local samples
#[test]
fn test_search_local_samples() {
    let sample_directory = PathBuf::from("tests/test_samples");
    let sample_name = "kick";
    let extension = "wav";
    let expected_paths = vec![
        PathBuf::from("tests/test_samples/kick1.wav"),
        PathBuf::from("tests/test_samples/kick2.wav"),
    ];

    assert_eq!(search_local_samples(sample_directory, sample_name, extension).unwrap(), expected_paths);
}

// Test the search for local samples when no samples are found
#[test]
fn test_search_local_samples_no_results() {
    let sample_directory = PathBuf::from("tests/test_samples");
    let sample_name = "snare";
    let extension = "wav";

    assert_eq!(search_local_samples(sample_directory, sample_name, extension).unwrap(), vec![]);
}

// Test the search for local samples with an invalid directory path
#[test]
fn test_search_local_samples_invalid_directory() {
    let sample_directory = PathBuf::from("invalid_directory");
    let sample_name = "kick";
    let extension = "wav";

    assert!(search_local_samples(sample_directory, sample_name, extension).is_err());


// Test the search for a sample pack online
#[test]
fn test_search_sample_pack_online() {
    let sample_name = "house";
    let expected_result = "House Sample Pack";

    assert_eq!(search_sample_pack_online(sample_name).unwrap(), expected_result);
}

// Test the search for a sample pack online when no results are found
#[test]
fn test_search_sample_pack_online_no_results() {
    let sample_name = "invalid_sample_name";

    assert_eq!(search_sample_pack_online(sample_name).unwrap(), "");
}

// Test the search for a sample on Splice
#[test]
fn test_search_sample_on_splice() {
    let sample_name = "kick";
    let expected_link = "https://splice.com/sounds/splice/kick-samples-pack";

    assert_eq!(search_sample_on_splice(sample_name).unwrap(), expected_link);
}

// Test the search for a sample on Splice when no results are found
#[test]
fn test_search_sample_on_splice_no_results() {
    let sample_name = "invalid_sample_name";

    assert_eq!(search_sample_on_splice(sample_name).unwrap(), "");
}

// Test the search for local samples
#[test]
fn test_search_local_samples() {
    let sample_directory = PathBuf::from("tests/test_samples");
    let sample_name = "kick";
    let extension = "wav";
    let expected_paths = vec![
        PathBuf::from("tests/test_samples/kick1.wav"),
        PathBuf::from("tests/test_samples/kick2.wav"),
    ];

    assert_eq!(search_local_samples(sample_directory, sample_name, extension).unwrap(), expected_paths);
}

// Test the search for local samples when no samples are found
#[test]
fn test_search_local_samples_no_results() {
    let sample_directory = PathBuf::from("tests/test_samples");
    let sample_name = "snare";
    let extension = "wav";

    assert_eq!(search_local_samples(sample_directory, sample_name, extension).unwrap(), vec![]);
}

// Test the search for local samples with an invalid directory path
#[test]
fn test_search_local_samples_invalid_directory() {
    let sample_directory = PathBuf::from("invalid_directory");
    let sample_name = "kick";
    let extension = "wav";

    assert!(search_local_samples(sample_directory, sample_name, extension).is_err());
}


// Test the search for a sample pack online
#[test]
fn test_search_sample_pack_online() {
    let sample_name = "house";
    let expected_result = "House Sample Pack";

    assert_eq!(search_sample_pack_online(sample_name).unwrap(), expected_result);
}
